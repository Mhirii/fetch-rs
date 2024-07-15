# fetch-rs
## About
A System Information Tool that i wrote to learn rust.

<p align="center"><img src="https://github.com/Mhirii/fetch-rs/blob/main/assets/screenshot.png" width="80%"></p>

>  the tool's name is no longer fetch-rs, use fetchr instead

## Usage
```bash
fetchr [options]
```

>  Options:  
>     --os  
>     --host  
>     --kernel  
>     --user  
>     --uptime  
>     --cpu  
>     --memory  
>     --packages  
>     --wm  
>     --session  
>     --terminal  
>     --logo  
> -h, --help      Print help  
> -V, --version   Print version  


  
## TODO list
- [x] Customization through CLI args
- [ ] Color Customization  
- [ ] Config File  
- [ ] Custom ASCII art support  
- [ ] Image support  

## Credits
Shoutout to [Gobidev](https://github.com/Gobidev) for his [pfetch](https://github.com/Gobidev/pfetch-rs), I was able to learn a lot and even used some of it's code, it was a great help. 

|  Tool   | Mean [ms]  | Min [ms] | Max [ms] |
| :---------------: | :--------: | :------: | :------: |
| neofetch(shell)   | 67.6 ± 16.9  |   53.9    |   101.1    |
| pfetch(rust)   | 64.4 ± 15.5  |   50.3    |   89.2    |
| fetchrs(rust)   | 2.5 ± 0.1  |   2.3    |   3.1    |


