# GitHub Copilot Instructions for Habitat Repository

## Repository Overview

Habitat is Chef's application automation framework that builds, deploys, and manages applications. This repository contains the core Habitat components written primarily in Rust.

## Repository Structure

```
habitat/
├── .expeditor/                   # Build and release automation scripts
│   ├── config.yml                # Expeditor configuration
│   └── scripts/                  # Build, test, and release scripts
├── .github/                      # GitHub workflows and templates
├── components/                   # Main Habitat components
│   ├── builder-api-client/       # Builder API client library
│   ├── builder-api/              # Builder API service
│   ├── common/                   # Shared utilities and common code
│   ├── core/                     # Core Habitat functionality
│   ├── hab/                      # Main hab CLI tool
│   ├── launcher/                 # Habitat launcher service
│   ├── pkg-export-container/     # Container export functionality
│   ├── pkg-export-tar/           # Tar export functionality
│   ├── plan-build/               # Plan building utilities
│   ├── studio/                   # Habitat Studio environment
│   └── sup/                      # Habitat Supervisor
├── support/                      # Build support files and scripts
├── test/                         # End-to-end tests
├── tools/                        # Development and build tools
├── Cargo.toml                    # Rust workspace configuration
├── Dockerfile                    # Docker build configuration
├── Justfile                      # Just command runner configuration
├── Makefile                      # Build automation
└── VERSION                       # Current version number
```

### Critical File Modification Rules


**🚨 NEVER MODIFY THESE FILES:**
- `.expeditor/config.yml` - This controls the release pipeline
- Files in `.expeditor/scripts/` unless specifically requested
- `POWERSHELL_VERSION` file - This is managed by the release process
- `RUST_NIGHTLY_VERSION` file - This is managed by the release process
- `RUSTFORMAT_VERSION` file - This is managed by the release process
- `VERSION` file - This is managed by the release process

## Task Implementation Workflow

### 1. Initial Setup and Jira Integration

When a Jira ID is provided:

1. **Fetch Jira Details**: Use the atlassian-mcp-server to fetch issue details
   ```bash
   # Use MCP server to get Jira issue information
   # Read the story description, acceptance criteria, and requirements
   ```

2. **Analyze Requirements**:
   - Read the Jira story thoroughly
   - Understand the scope and acceptance criteria
   - Identify which components need modification

3. **Provide Summary**: Give a clear summary of what needs to be implemented

### 2. Pre-Implementation Analysis

1. **Component Identification**: Determine which Habitat components are affected
2. **Impact Assessment**: Analyze potential breaking changes
3. **Test Strategy**: Plan unit tests and integration tests needed
4. **File Review**: Identify files that will be modified (avoiding prohibited files)

### 3. Implementation Phase

1. **Code Implementation**:
   - Follow Rust best practices and Habitat coding standards
   - Ensure thread safety and error handling
   - Use appropriate logging levels
   - Follow existing patterns in the codebase

2. **Unit Test Creation**:
   - Create comprehensive unit tests for new functionality
   - Ensure test coverage remains > 80%
   - Use existing test patterns and utilities
   - Test error conditions and edge cases

3. **Integration Testing**:
   - Run relevant integration tests
   - Verify compatibility with existing functionality

### 4. Quality Assurance

1. **Code Coverage**: Verify coverage is > 80%
   ```bash
   cargo tarpaulin --out html --output-dir coverage/
   ```

2. **Linting and Formatting**:
   ```bash
   cargo clippy --all-targets --all-features
   cargo fmt --check
   ```

3. **Build Verification**:
   ```bash
   cargo build --all-targets
   cargo test --all
   ```

### 5. Git Workflow, PR Creation, and JIRA Update

1. **Branch Creation**: Use Jira ID as branch name
   ```bash
   git checkout -b <JIRA-ID>
   ```

2. **Commit Changes**:
   ```bash
   git add .
   git commit -s -m "<JIRA-ID>: Brief description of changes"
   ```

3. **Push and Create PR**:
   ```bash
   git push origin <JIRA-ID>
   gh pr create --title "<JIRA-ID>: Brief title" --body "<HTML formatted description>" --label "ai-assisted"
   ```

4. **PR Description Format** (HTML):
   ```html
   <h2>Summary</h2>
   <p>Brief description of changes made</p>

   <h2>Changes Made</h2>
   <ul>
     <li>Change 1</li>
     <li>Change 2</li>
   </ul>

   <h2>Testing</h2>
   <ul>
     <li>Unit tests added/updated</li>
     <li>Integration tests verified</li>
     <li>Coverage maintained > 80%</li>
   </ul>

   <h2>AI Assistance</h2>
   <p>This work was completed with AI assistance following Progress AI policies</p>

   <h2>Jira Link</h2>
   <p><a href="link-to-jira-issue">JIRA-ID</a></p>
   ```

### 6. Update JIRA Ticket (MANDATORY)

After successful PR creation, you MUST update the JIRA ticket to indicate AI assistance was used:

1. **Use Atlassian MCP Server**: Use the `mcp_atlassian-mcp_editJiraIssue` tool to update the ticket
2. **Target Field**: Update `customfield_11170` ("Does this Work Include AI Assisted Code?") 
3. **Required Value**: Set to "Yes"
4. **Correct Format**: Use exact format `{"customfield_11170": {"value": "Yes"}}`
5. **Verification**: Confirm the field update was successful

**Example MCP Call**:
```
mcp_atlassian-mcp_editJiraIssue with:
- cloudId: [your-atlassian-cloud-id]
- issueIdOrKey: [JIRA-ID]  
- fields: {"customfield_11170": {"value": "Yes"}}
```

**CRITICAL**: This step is mandatory for all AI-assisted work and must be completed before declaring the task finished.

## Prompt-Based Workflow

### Step-by-Step Process

After each step, provide:
1. **Summary of completed step**
2. **Next step description**
3. **Remaining steps overview**
4. **Prompt asking if you want to continue**

Example format:
```
✅ **Step X Completed**: [Description of what was done]

🔄 **Next Step**: [Description of next step]

📋 **Remaining Steps**:
- Step Y: [Description]
- Step Z: [Description]

❓ **Ready to continue with the next step?** (Yes/No)
```

### Complete Workflow Example

1. **Jira Analysis** → Summary + "Continue with component analysis?"
2. **Component Analysis** → Summary + "Continue with implementation?"
3. **Implementation** → Summary + "Continue with testing?"
4. **Testing** → Summary + "Continue with quality checks?"
5. **Quality Assurance** → Summary + "Continue with PR creation?"
6. **PR Creation** → Summary + "Continue with JIRA update?"
7. **JIRA Update** → Summary + "Task completed!"

## Development Guidelines

### Code Standards

1. **Rust Best Practices**:
   - Use `#[derive(Debug)]` for structs
   - Implement proper error handling with `Result<T, E>`
   - Use `serde` for serialization when needed
   - Follow naming conventions (snake_case for functions/variables)

2. **Testing Standards**:
   - Unit tests in the same file as implementation (when appropriate)
   - Integration tests in `tests/` directory
   - Mock external dependencies
   - Test both success and failure cases

3. **Documentation**:
   - Add rustdoc comments for public APIs
   - Update relevant markdown documentation
   - Include examples in documentation when helpful

### Environment Setup

1. **Required Tools**:
   ```bash
   # Ensure these are installed
   rustc --version  # Rust compiler
   cargo --version  # Cargo package manager
   hab --version    # Habitat CLI
   gh --version     # GitHub CLI
   ```

2. **Build Environment**:
   ```bash
   # Set up development environment
   export HAB_LICENSE=accept-no-persist
   export RUST_LOG=debug
   ```

### Testing Commands

```bash
# Run all tests
cargo test --all

# Run specific component tests
cargo test -p habitat_core

# Run with coverage
cargo tarpaulin --all --out html

# Lint code
cargo clippy --all-targets --all-features

# Format code
cargo fmt
```

## MCP Server Integration

### Atlassian MCP Server

When working with Jira issues, use the atlassian-mcp-server:

1. **Fetch Issue Details**:
   - Get issue summary, description, and acceptance criteria
   - Read comments and linked issues
   - Understand priority and labels

2. **Update Issue Status** (if required):
   - Transition issue status as work progresses
   - Add comments with implementation details

## GitHub CLI Configuration

Ensure GitHub CLI is properly authenticated:

```bash
# Check authentication status
gh auth status

# Login if needed (without .profile references)
gh auth login --web
```

## Final Checklist

Before completing any task:

- [ ] Jira issue requirements fully understood and implemented
- [ ] No prohibited files modified
- [ ] Unit tests created with >80% coverage
- [ ] All tests passing
- [ ] Code properly formatted and linted
- [ ] Branch created with Jira ID
- [ ] PR created with HTML description and "ai-assisted" label
- [ ] JIRA ticket updated with customfield_11170 set to "Yes"
- [ ] All steps completed with user confirmation

## Emergency Contacts

For questions about:
- **Build/Release Issues**: Check `.expeditor/` scripts
- **Component Architecture**: Review `components/` README files
- **Testing**: Check existing test patterns in `tests/` directories
- **Habitat Concepts**: Review documentation in `components/docs-chef-io/`

---

**Remember**: Always follow the prompt-based workflow, asking for confirmation before proceeding to each next step, and maintain the >80% test coverage requirement throughout development.