
import random

codes = [
    100,
    101,
    102,
    200,
    201,
    203,
    204,
    205,
    206,
    207,
    208,
    226,
    300,
    301,
    302,
    303,
    304,
    305,
    306,
    307,
    308,
    400,
    401,
    402,
    403,
    404,
    405,
    406,
    407,
    408,
    409,
    410,
    411,
    412,
    413,
    414,
    415,
    416,
    417,
    418,
    421,
    422,
    423,
    424,
    426,
    428,
    429,
    431,
    451,
    500,
    501,
    502,
    503,
    504,
    505,
    506,
    507,
    508,
    510,
    511,
]

for filenum in range(0, 10):
    filename = "./example-logfile-{0:04d}".format(filenum)

    print(filename)
    msg = ('123.65.150.10 - - [23/Aug/2010:{hour:02d}:{minute:02d}:'
           '{second:02d} +0000] "POST /wordpress3/wp-admin/admin-ajax'
           '.php HTTP/1.1" {code} 2 "http://www.example.com/wordpress3'
           '/wp-admin/post-new.php" "Mozilla/5.0 (Macintosh; U; Intel '
           'Mac OS X 10_6_4; en-US) AppleWebKit/534.3 (KHTML, like '
           'Gecko) Chrome/6.0.472.25 Safari/534.3"' + "\n")

    with open(filename, 'w') as fp:
        for hour in range(0, 24):
            for minute in range(0, 60):
                for second in range(0, 60):
                    for code in random.sample(codes, 23):
                        fp.write(msg.format(hour=hour, minute=minute,
                                            second=second, code=code))
