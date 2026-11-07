#!/usr/bin/env bash
set -euo pipefail

database_path="${DATABASE_PATH:?Set DATABASE_PATH the the directory where the DB file is stored, e.g. /var/lib/stream-stash/stream_stash.db}"
backup_remote="${BACKUP_REMOTE:?Set BACKUP_REMOTE to an rclone destination, e.g. gdrive:stream-stash}"
backup_dir="${BACKUP_DIR:?Set BACKUP_DIR to the directory where DB backups should be stored, e.g. /var/lib/stream-stash/backups}"

# systemd turns the <N> prefix into the journal priority (3 = error, 6 = info)
log_error() {
    echo "<3>$*" >&2
}

log_info() {
    echo "<6>$*"
}

# Runs rclone and logs an error to the journal if it fails
run_rclone() {
    local description="$1"
    shift

    local output
    if ! output="$(rclone "$@" 2>&1)"; then
        log_error "Google Drive backup failed: $description (rclone $1): $output"
        return 1
    fi

    printf '%s' "$output"
}

if [[ ! -r "$database_path" ]]; then
    log_error "Database is not readable: $database_path"
    exit 1
fi

mkdir -p "$backup_dir"
snapshot="$(mktemp "$backup_dir/.snapshot.XXXXXX")"
trap 'rm -f -- "$snapshot"' EXIT

sqlite3 "$database_path" ".backup '$snapshot'"
integrity_result="$(sqlite3 "$snapshot" 'PRAGMA integrity_check;')"
if [[ "$integrity_result" != "ok" ]]; then
    log_error "SQLite integrity check failed: $integrity_result"
    exit 1
fi

snapshot_hash="$(sha256sum "$snapshot" | cut -d ' ' -f 1)"
timestamp="$(date -u +%Y-%m-%dT%H)"
day="${timestamp:8:2}"
hour="${timestamp:11:2}"

upload_tier_if_changed() {
    local tier="$1"
    local keep="$2"
    local filename="$3"
    local hash_file="$backup_dir/.${tier}.sha256"
    local remote_dir="$backup_remote/$tier"

    if [[ ! -f "$hash_file" || "$(cat "$hash_file")" != "$snapshot_hash" ]]; then
        run_rclone "uploading $tier snapshot $filename" copyto "$snapshot" "$remote_dir/$filename" >/dev/null || return 1
        printf '%s\n' "$snapshot_hash" > "$hash_file.tmp"
        mv -f -- "$hash_file.tmp" "$hash_file"
        log_info "Uploaded $tier snapshot: $filename"
    else
        log_info "No database changes for $tier; skipping upload"
    fi

    local listing
    listing="$(run_rclone "listing $tier snapshots for rotation" lsf --files-only --include '*.db' "$remote_dir")" || return 1
    listing="$(LC_ALL=C sort -r <<< "$listing")"
    if [[ -n "$listing" ]]; then
        local -a files
        mapfile -t files <<< "$listing"
        local index
        for ((index = keep; index < ${#files[@]}; index++)); do
            run_rclone "deleting old $tier snapshot ${files[index]}" deletefile "$remote_dir/${files[index]}" >/dev/null || return 1
        done
    fi
}

failed=0

upload_tier_if_changed son 24 "$timestamp.db" || failed=1

if [[ "$hour" == "00" ]]; then
    upload_tier_if_changed father 30 "${timestamp%%T*}.db" || failed=1
fi

if [[ "$hour" == "00" && "$day" == "01" ]]; then
    upload_tier_if_changed grandfather 12 "${timestamp:0:7}.db" || failed=1
fi

exit "$failed"
