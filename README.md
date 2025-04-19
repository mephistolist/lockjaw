# lockjaw 2.0
A web spider in Rust that helps locate forms and hide tracks. This version was almost a complete rewrite of verion 1.0. That version would spider and follow any link often branching out to sites not related to the target. While that can be useful for search engines, it was counter-intuitive to mapping out an organization. So now unrelated links will only be spidered or recorded if requested and not followed in any case. 

Its easy for most people to use a VPN or tor and hide their ip address. This code does not do that, but focues on spoofing the X-Forwarded-For, X-Originating-IP, X-Remote-IP and X-Remote-Addr headers instead. Even if an ip is spoofed, these headers can give away the user's true ip address. Most tor nodes and some VPNs will claim to strip out these headers, but if you aren't there, you don't know. The option to set a user-agent is also given to appear as a normal web browser or whatever string you wish to send to the host.

You can build this with:

```
sudo make install
```

Usage can be found with -h or --help:

```
$ lockjaw -h             
Lockjaw Spider 2.0
By Mephistolist
Web spider in Rust that helps to hide tracks.
USAGE:
    lockjaw [FLAGS] [OPTIONS] --database <DB_FILE> --url <URL>
FLAGS:
    -e, --everything    Spider all URLs but only follow same-domain or subdomain links
    -h, --help          Prints help information
    -s, --subs          Enable crawling subdomains
    -V, --version       Prints version information
OPTIONS:
    -d, --database <DB_FILE>         Sets the SQLite database file
    -x, --spoof <Spoofed_IP>         Sets the spoofed IP for headers
    -t, --time <SECONDS>             Delay between requests (1-300 seconds) [default: 1]
    -u, --url <URL>                  Sets the starting URL for the spider
    -a, --user-agent <USER_AGENT>    Sets the user agent string [default: Lockjaw Spider 2.0]
```
You can run it like this:

```
$ lockjaw -u http://localhost -x 127.0.0.1 -d mine.db -a "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36"
```
Besides the console output, this will be written to an sqlite3 database. You can view everything was found in the database specified with the following:

```
$ sqlite3 mine.db                                                                   
SQLite version 3.44.2 2023-11-24 11:41:44
Enter ".help" for usage hints.
sqlite> SELECT * FROM links;
```
You can refine the search for only urls with 200 response codes, or any response code, like this:

```
select url from links where status_code = '200';
```

Or just display links where form tags were found on the page like this:

```
select url from links where has_form = 'y';
```

From here a pen tester may inspect these urls with these forms to craft XSS or SQL injections. 

The laws on spoofing, spidering web forms or pen testing may vary greatly country to country. If you are unsure, please refrain until you are familar with local laws.

This sofware has been tested and confirmed to work with Fedora, Debian and FreeBSD. 
