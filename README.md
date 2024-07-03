# concoot.exe

Personal tool for uploading codebase into LLMs for querying.

Installing
```
cargo install --path .
```

You can also do
```
cargo install --git https://github.com/stevenhuyn/concoot.git
```

Now you can use 
```
concoot
```

How to copy to clipboard? (Windows + CMD)
```cmd
concoot c | clip
```

And it'll spit out an `output.txt` that you can put inside your LLM to help you query your own codebase!