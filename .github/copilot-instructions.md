# GitHub Copilot Instructions for Habitat Repository

This file contains comprehensive instructions for GitHub Copilot to effectively work with the Habitat repository.

## Repository Overview

Habitat is Chef's application automation framework that builds, deploys, and manages applications. This repository contains the core Habitat components written primarily in Rust.

## Repository Structure

The Habitat repository is organized as follows:

### Root Level
```
habitat/
├── .github/              # GitHub configurations and workflows
├── .vscode/              # VS Code configurations including mcp.json
├── components/           # Main Rust components and crates
├── test/                # Test suites (end-to-end, fixtures, integration)
├── test-services/       # Test service implementations
├── tools/               # Utility tools and scripts
├── support/             # Build and development support files
├── dev-docs/            # Developer documentation
├── images/              # Documentation images and diagrams
├── Cargo.toml           # Workspace configuration
├── Cargo.lock           # Dependency lock file
├── README.md            # Main project documentation
├── BUILDING.md          # Build instructions
├── CONTRIBUTING.md      # Contribution guidelines
└── ...                  # Other configuration and documentation files
```

### Components Directory Structure
```
components/
├── builder-api-client/  # Builder API client library
├── butterfly/           # Gossip protocol implementation
├── common/              # Shared utilities and common code
├── core/                # Core Habitat functionality
├── hab/                 # Main hab CLI tool
├── launcher/            # Habitat launcher service
├── launcher-client/     # Launcher client
├── launcher-protocol/   # Launcher protocol definitions
├── pkg-export-container/ # Container export functionality
├── pkg-export-tar/      # Tar export functionality
├── rst-reader/          # RST file reader
├── studio/              # Habitat Studio environment
├── sup/                 # Habitat Supervisor
├── sup-client/          # Supervisor client
├── sup-protocol/        # Supervisor protocol definitions
├── welcome-cli/         # Welcome CLI utility
├── win-users/           # Windows user management
└── windows-service/     # Windows service implementation
```

### Test Structure
```
test/
├── end-to-end/          # End-to-end test suites
├── fixtures/            # Test fixtures and data
└── integration/         # Integration tests
```

## Critical File Modification Restrictions

### ⚠️ IMPORTANT: Do NOT modify *.codegen.go files
- **NEVER** modify any files with the pattern `*.codegen.go`
- These files are automatically generated and should not be manually edited
- Any changes to these files will be overwritten during code generation
- If you need to modify generated code, find the source templates or generators instead

## MCP Server Integration

### Atlassian MCP Server Usage
When a Jira ID is provided in any task:

1. **Use the atlassian-mcp-server** configured in `.vscode/mcp.json`
2. **Fetch Jira issue details** using the MCP server tools
3. **Read the story/description** thoroughly to understand requirements
4. **Implement the task** according to the Jira issue specifications
5. **Reference the Jira ID** in commit messages and PR descriptions

### MCP Server Configuration
The repository includes an `atlassian-mcp-server` configuration:
```json
{
  "servers": {
    "atlassian-mcp-server": {
      "url": "https://mcp.atlassian.com/v1/sse",
      "type": "http"
    }
  },
  "inputs": []
}
```

## Testing Requirements

### Unit Test Coverage
- **ALWAYS** create comprehensive unit test cases for your implementation
- **Maintain test coverage > 80%** at all times
- Place unit tests in the appropriate `tests/` directory within each component
- Use Rust's built-in testing framework with `#[cfg(test)]` modules
- Run tests with `cargo test` to verify coverage

### Test Structure Guidelines
- Create tests for both happy path and error scenarios
- Include edge cases and boundary conditions
- Mock external dependencies appropriately
- Use descriptive test names that explain what is being tested

## Pull Request Workflow

### Branch Creation and PR Process
When prompted to create a PR for changes:

1. **Branch Naming**: Use the Jira ID as the branch name (e.g., `HAB-123`)
2. **Use GitHub CLI** for all Git operations:
   ```bash
   # Create and switch to branch
   gh repo clone habitat-sh/habitat  # if not already cloned
   git checkout -b HAB-123
   
   # Make your changes and commit
   git add .
   git commit -m "HAB-123: Brief description of changes"
   
   # Push branch
   git push origin HAB-123
   
   # Create PR with HTML-formatted description
   gh pr create --title "HAB-123: Title" --body "PR description with HTML tags"
   ```

3. **PR Description Format**: Use HTML tags for formatting:
   ```html
   <h3>Summary</h3>
   <p>Brief summary of changes made</p>
   
   <h3>Changes</h3>
   <ul>
   <li>Change 1</li>
   <li>Change 2</li>
   </ul>
   
   <h3>Testing</h3>
   <p>Description of tests added/run</p>
   
   <h3>Jira Issue</h3>
   <p>Resolves: HAB-123</p>
   ```

4. **Add Required Labels**: Always add the label `runtest:all:stable` to PRs:
   ```bash
   gh pr edit HAB-123 --add-label "runtest:all:stable"
   ```

### Authentication
- Use GitHub CLI authentication (avoid references to ~/.profile)
- Authenticate with: `gh auth login`
- Follow the interactive prompts for authentication

## Prompt-Based Task Management

### Step-by-Step Approach
All tasks should be performed in a prompt-based manner:

1. **After each step**: Provide a clear summary of what was completed
2. **Next step preview**: Explain what the next step will be
3. **Remaining steps**: List what other steps are left to complete
4. **Confirmation prompt**: Ask "Do you want to continue with the next step?"

### Example Workflow Communication
```
✅ Step 1 Completed: Created new component structure
📋 Next Step: Implement core functionality
🔄 Remaining Steps: Write tests, Update documentation, Create PR
❓ Do you want to continue with the next step?
```

## Complete Task Implementation Workflow

### 1. Task Analysis Phase
- [ ] Read and understand the task requirements
- [ ] If Jira ID provided: Fetch issue details using atlassian-mcp-server
- [ ] Analyze impact on existing codebase
- [ ] Identify components that need modification
- [ ] **Verify no *.codegen.go files will be affected**

### 2. Planning Phase
- [ ] Create implementation plan
- [ ] Identify test scenarios
- [ ] Plan for >80% test coverage
- [ ] Determine if any dependencies are needed

### 3. Implementation Phase
- [ ] Create/modify necessary components
- [ ] Follow Rust best practices and existing code patterns
- [ ] Ensure compatibility with existing APIs
- [ ] **Avoid modifying any *.codegen.go files**

### 4. Testing Phase
- [ ] Write comprehensive unit tests
- [ ] Ensure >80% test coverage
- [ ] Run all existing tests to verify no regressions
- [ ] Test both success and failure scenarios
- [ ] Run `cargo test` and verify all tests pass

### 5. Documentation Phase
- [ ] Update relevant documentation
- [ ] Add/update code comments
- [ ] Update README if necessary
- [ ] Document any new APIs or changes

### 6. PR Creation Phase
- [ ] Create branch using Jira ID (if applicable)
- [ ] Commit changes with descriptive messages
- [ ] Push to remote branch using `gh` CLI
- [ ] Create PR with HTML-formatted description
- [ ] Add `runtest:all:stable` label
- [ ] Request review from appropriate maintainers

### 7. Final Verification
- [ ] Verify all tests pass in CI
- [ ] Ensure code coverage remains >80%
- [ ] Confirm no restricted files were modified
- [ ] Validate PR description includes all required information

## Prohibited Modifications

### Files/Patterns to NEVER Modify
- `*.codegen.go` - Auto-generated Go files
- `Cargo.lock` - Unless adding new dependencies
- `.git/` directory contents
- CI/CD workflow files without explicit permission

### Safe to Modify
- Source code in `components/*/src/`
- Test files in `components/*/tests/` or `test/`
- Documentation files (*.md)
- Configuration files (with caution)
- `Cargo.toml` files for adding dependencies

## Local Repository Operations

All tasks will be performed on the local repository. Ensure:
- Working directory is set to the repository root
- All changes are committed locally before pushing
- Use relative paths when referencing files
- Test locally before creating PRs

## Quality Standards

- **Code Quality**: Follow Rust idioms and existing patterns
- **Test Coverage**: Maintain >80% coverage
- **Documentation**: Keep documentation up-to-date
- **Backwards Compatibility**: Preserve existing API contracts
- **Performance**: Consider performance implications of changes

---

**Remember**: Always ask for confirmation before proceeding to the next step, and provide clear summaries of completed work and remaining tasks.