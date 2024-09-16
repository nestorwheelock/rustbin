
# rustbin

**rustbin** is a Rust-based tool that automates the deployment of a Rust binary from the `target/release` directory to `/usr/local/bin`, making it globally accessible on the system.

## Program Overview

The **rustbin** program handles the following tasks:

1. **Command-line Argument Handling**: 
   - The program expects exactly one command-line argument: the name of the binary to be deployed.
   - If no argument is provided, it prints the correct usage: 
     ```
     Usage: rustbin <binary_name>
     ```
     and exits.

2. **Binary Existence Check**:
   - It verifies whether the specified binary exists in the `target/release/` directory.
   - If the binary does not exist, it outputs an error message and exits.

3. **Remove Existing Binary**:
   - If a binary with the same name already exists in `/usr/local/bin`, it removes it before deploying the new one. This ensures that the old version is replaced.

4. **Copy and Deploy**:
   - The program copies the binary from `target/release` to `/usr/local/bin`, making it accessible system-wide.

5. **Make the Binary Executable**:
   - After copying, it sets the binary's permissions to executable using the `chmod +x` command.

6. **Success Message**:
   - Once the deployment is successful, it prints a message indicating that the binary is now available in `/usr/local/bin`.

## Usage Example:

To deploy a binary using **rustbin**, run the following command:

```bash
./rustbin <binary_name>
```

For example, to deploy `my_program`:

```bash
./rustbin my_program
```

This command will:
1. Check if `target/release/my_program` exists.
2. Remove any existing version in `/usr/local/bin/my_program`.
3. Copy `my_program` to `/usr/local/bin/`.
4. Make it executable.

### Example Output:

```
Removing existing binary from /usr/local/bin/my_program
Deploying my_program to /usr/local/bin
my_program has been successfully deployed to /usr/local/bin
```

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for more details.
