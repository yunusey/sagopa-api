# Sagopa API

Yes, you read that right. This is an API I've written in [Shuttle](https://www.shuttle.rs/) with [Axum](https://axum.rs/) and [Yew](https://yew.rs/).

## Why?
I am not a web developer, but sometimes I need to do some API calls, and that [Lorem Ipsum](https://www.lipsum.com/) is too **boring**. As a huge fan of [Sagopa Kajmer](https://www.youtube.com/channel/UC-7fJF4x8vV7Ynyp47h-Zow), I thought why not just create one myself containing some quotes of him while learning awesome frameworks like Yew, Axum, and Shuttle.

## How do you use it?
I am planning to leave several examples for some languages to use it in the future, but basically, you just make a REST API call and you are good to go.

_PS: You probably don't care about `id`, `album_id`, and `song_id` field. So, just ignore them!_

### Get a random quote

```bash
# Get a random quote
curl -X GET "https://sagopa-api.shuttleapp.rs/get/random/quote"
```

Response:
```json
{
    "song_id": 2,
    "quote": "...",
    "id": 3
}
```

### Get all quotes

```bash
# Get all quotes
curl -X GET "https://sagopa-api.shuttleapp.rs/get/quote"
```

Response:
```json
[
    {
        "song_id": 2,
        "quote": "...",
        "id": 3
    },
    ...
]
```

### See all the albums

```bash
# See all the albums
curl -X GET "https://sagopa-api.shuttleapp.rs/get/albums"
```

Response:
```json
[
    {
        "name": "Kağıt Kesikleri",
        "id": 1
    }
]
```

### See all the songs

```bash
# See all the songs
curl -X GET "https://sagopa-api.shuttleapp.rs/get/songs"
```

Response:
```json
[
    {
        "album_id": 1,
        "name": "Halen",
        "id": 1
    },
    ...
]
```

## Contributing?

You want to contribute? You're awesome! Well, if you want to contribute to the source code, you can explore the code, open issues, create PRs, whatever you want. If you want to contribute to the service by adding quotes, which I would really appreciate, the only thing you need to do is to listen to more, go to [Sagopa API](https://sagopa-api.shuttleapp.rs/), and enter your quotes! Yep, that's pretty much all.

## License

Of course, this project is MIT licensed. Use it as you wish.

## Credits
- [Axum](https://axum.rs/)
- [Yew](https://yew.rs/)
- [Shuttle](https://www.shuttle.rs/)

## Final Words
Currently, the API doesn't have so many quotes, I will add as I listen to more--if you want to make this process faster, why not you add some that you like? Thanks for visiting!
