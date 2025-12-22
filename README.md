# sentry-cli

A command-line tool for managing Sentry issues.

## Installation

### From source

```bash
cargo install --path .
```

### Build from source

```bash
git clone https://github.com/yourusername/sentry-cli
cd sentry-cli
cargo build --release
```

The binary will be at `target/release/sentry-cli`.

## Configuration

### Authentication

Set your Sentry auth token via environment variable (recommended):

```bash
export SENTRY_AUTH_TOKEN="sntrys_..."
export SENTRY_ORG="your-organization"
```

Or create a config file:

```bash
sentry-cli config init
# Edit ~/.config/sentry-cli/config.toml
```

Config file format (`~/.config/sentry-cli/config.toml`):

```toml
default_org = "your-organization"
server_url = "https://sentry.io"  # or your self-hosted URL
auth_token = "sntrys_..."
default_project = "your-project"
```

**Priority order**: CLI flags > environment variables > config file > defaults

### Self-hosted Sentry

For self-hosted Sentry instances:

```bash
sentry-cli --server https://sentry.yourcompany.com issues list
```

Or set in config:

```toml
server_url = "https://sentry.yourcompany.com"
```

## Usage

### List Issues

```bash
# List all unresolved issues
sentry-cli issues list

# Filter by project
sentry-cli issues list --project backend

# Filter by status
sentry-cli issues list --status resolved

# Custom search query
sentry-cli issues list --query "is:unresolved level:error"

# JSON output (for scripting)
sentry-cli issues list --output json

# Fetch all pages
sentry-cli issues list --all --limit 100
```

### View Issue Details

```bash
sentry-cli issues view 1234567890
sentry-cli issues view PROJ-123

# JSON output
sentry-cli issues view 1234567890 --output json
```

### Resolve Issues

```bash
# Resolve a single issue
sentry-cli issues resolve 1234567890

# Resolve multiple issues
sentry-cli issues resolve 1234567890 1234567891

# Resolve in specific release
sentry-cli issues resolve 1234567890 --in-release 1.2.3

# Resolve in next release
sentry-cli issues resolve 1234567890 --in-next-release
```

### Unresolve Issues

```bash
sentry-cli issues unresolve 1234567890
```

### Assign Issues

```bash
# Assign to user
sentry-cli issues assign 1234567890 --to user@example.com

# Assign to team
sentry-cli issues assign 1234567890 --to team:backend

# Unassign
sentry-cli issues assign 1234567890 --unassign
```

### Ignore Issues

```bash
# Ignore indefinitely
sentry-cli issues ignore 1234567890

# Ignore for 24 hours (1440 minutes)
sentry-cli issues ignore 1234567890 --duration 1440

# Ignore until 100 more events
sentry-cli issues ignore 1234567890 --count 100

# Ignore until escalating
sentry-cli issues ignore 1234567890 --until-escalating
```

### Delete Issues

```bash
# Delete with confirmation prompt
sentry-cli issues delete 1234567890

# Skip confirmation
sentry-cli issues delete 1234567890 --confirm

# Delete multiple
sentry-cli issues delete 1234567890 1234567891 --confirm
```

### Merge Issues

```bash
# Merge issues into a primary issue
sentry-cli issues merge 1234567890 1234567891 1234567892
```

### Configuration Management

```bash
# Create config file
sentry-cli config init

# Show current config
sentry-cli config show

# Set config values
sentry-cli config set default_org my-org
sentry-cli config set auth_token sntrys_...
```

## Global Options

```
--server <URL>     Sentry server URL (default: https://sentry.io)
--org <ORG>        Organization slug
--token <TOKEN>    Auth token
-v, --verbose      Enable verbose output (shows API requests)
-h, --help         Print help
-V, --version      Print version
```

## Environment Variables

| Variable | Description |
|----------|-------------|
| `SENTRY_AUTH_TOKEN` | Authentication token |
| `SENTRY_ORG` | Default organization slug |
| `SENTRY_SERVER_URL` | Sentry server URL |
| `SENTRY_PROJECT` | Default project slug |

## Examples

### Scripting with JSON output

```bash
# Get all issue IDs
sentry-cli issues list --output json | jq -r '.[].id'

# Count issues by status
sentry-cli issues list --all --output json | jq 'group_by(.status) | map({status: .[0].status, count: length})'

# Resolve all issues matching a query
sentry-cli issues list --query "is:unresolved browser:Chrome" --output json | \
  jq -r '.[].id' | \
  xargs sentry-cli issues resolve
```

### Verbose mode for debugging

```bash
sentry-cli -v issues list
# [verbose] Server: https://sentry.io/
# [verbose] Organization: my-org
# [verbose] GET https://sentry.io/api/0/organizations/my-org/issues/...
# [verbose] Response: 200 OK
```

## License

MIT
