# PWSearch
## a tool for searching for breached passwords locally

The great Troy Hunt, creator of ["Have I Been Pwned?"](https://haveibeenpwned.com), has recently released a database of over 500 million compromised / leaked passwords, [here](https://haveibeenpwned.com/Passwords).

You don't **have to** download the entire database, as the site provides a password search functionality. Nevertheless, regardless of how awesome Troy is, as a rule I never trust a 3rd-party website with any of my passwords except if it's the one I use to log in to that specific site.

So, I'm a bit reluctant to submit my password (or even the SHA1 of it) just in order to have it checked. After all, if it hasn't yet been compromised, that opportunity would well open up by just sending it around on the Interwebz…

Hence, I have created this little utility that you can use for checking your beloved `p4ssw0rd` hashes locally. Here's how to use it:

1. Download the "Version 2" database. The filename should be `pwned-passwords-2.0.txt.7z`
2. Move it to the root directory of this repository.
3. Run the `make_searchable.sh` script in this directory. It will extract the archive, sort it (in order to be able to perform binary search on it), and edit it so that only the hashes remain in the file. (Again, this makes it possible for the search tool to perform a binary search.)

    **Keep in mind that the script will delete any intermediate files as soon as they are not needed anymore, including the original 7z archive.** This is done because the files are *huge,* so otherwise you may well end up running out of disk space before the conversion has a chance to complete.

4. `cargo run` - this will build and run the actual search program. You will need a Rust toolchain for this step.
5. When it asks for your password, specify it. Don't worry – you can read the source code and see for yourself that it doesn't do anything funny with it. :P That's kind of **the point** of using this tool on your own machine.
6. The output will be either "YOU ARE PWNED" or "You are probably safe."

## License

MIT License
