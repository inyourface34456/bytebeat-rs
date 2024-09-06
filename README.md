This will generate a "perfect" redition of bytebeat in perfect quality.  There is some bugs (samples that I grabbed off of [Dolchan.net](https://dollchan.net) dont really sound as they should, if anyone wants to help, submit a PR and i might get to it).  Can do all kinds of byte beat.

## How to use:
make a javascript file with any name that you want.
copy in this template:
```javascript
// should return <sample rate in hz>,<number of channels>,<duration in secs>,<kind, only 0, 1, or 2>
// 0=unsigned bytebeat
// 1=signed bytebeat
// 2=floatbeat (0-1 range)
function d() {
    return "8000,1,60,1"
}

//takes in an intager, t, returns a number
function t(t) {
    return Math.sin(t);
}
```
run the program like this: `bytebeat-rs.exe <path to js file>`
listen to `sound.wav`
