### Sysquery

Sysquery is a CLI tool to get information about your system. With the first release of this CLI tool, you can scan for large files in your working directory, get a system digest of what the current resources are on your machine, check what processes are running, and check where network traffic is occurring. 

### Installation

`cargo install sysquery`

### Usage

Pick the `n` largest files in your directory to be returned:
`sysquery largefiles <NUM_FILES>`

Display the `n` most expensive processes by memory: 
`sysquery process`

Show system information: 
`sysquery digest`

For more commands:
`sysquery --help`
