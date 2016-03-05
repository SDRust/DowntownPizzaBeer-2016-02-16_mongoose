## San Diego Rust Meetup - February 16th, 2016 ##

Tonight's exercise was to parse the output of https://en.wikipedia.org, and
count how many HTML `<div>` tags are included on that page.

The Meetup group was split in to two teams, and each team was assigned a Rust
HTTP library to demo and use in their final project code.

After some time spent hacking, this repo is the result of "Team Mongoose", which
was assigned a non-**hyper** HTTP crate. We tried a MIO version of some HTTP crate
but unfortunately that was not mature enough. We settled on using Rust bindings for cURL.

Enjoy!
