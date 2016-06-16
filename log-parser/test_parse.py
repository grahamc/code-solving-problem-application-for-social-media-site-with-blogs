
import parse
import unittest


class TestCodesByDateTimeCounter(unittest.TestCase):
    def parse_lines(self):
        return [
            (('15/Jun/2016:18:10', '200'), '::1 - - [15/Jun/2016:18:10:23 -0400] "OPTIONS * HTTP/1.0" 200 110 "-" "Apache/2.4.7 (Ubuntu) PHP/5.5.9-1ubuntu4.5 OpenSSL/1.0.1f mod_wsgi/3.4 Python/2.7.6 (internal dummy connection)"'),  # noqa
            (('15/Jun/2016:18:12', '200'), '127.0.0.1 - - [15/Jun/2016:18:12:04 -0400] "GET /server-status?auto HTTP/1.1" 200 517 "-" "Linode Longview 1.1.4 client: 77D66E2B-F813-5205-EC7357DF2EA499AC"'),  # noqa
            (('15/Jun/2016:18:36', '404'), '62.138.2.243 - - [15/Jun/2016:18:36:28 -0400] "GET /robots.txt HTTP/1.0" 404 372 "-" "Mozilla/5.0 (compatible; MJ12bot/v1.4.5; http://www.majestic12.co.uk/bot.php?+)"'),  # noqa
            (('10/Oct/2000:13:55', '200'), '127.0.0.1 - frank [10/Oct/2000:13:55:36 -0700] "GET /apache_pb.gif HTTP/1.0" 200 2326 "http://www.example.com/start.html" "Mozilla/4.08 [en] (Win98; I ;Nav)"'),  # noqa
        ]

    def test_parse_regex(self):
        counter = parse.CodesByDateTimeCounter()

        for ((date, code), line) in self.parse_lines():
            self.assertEquals((date, code), counter.parse_line_regex(line))

    def test_parse_scan(self):
        counter = parse.CodesByDateTimeCounter()

        for ((date, code), line) in self.parse_lines():
            self.assertEquals((date, code), counter.parse_line_scan(line))

    def test_parse_split(self):
        counter = parse.CodesByDateTimeCounter()

        for ((date, code), line) in self.parse_lines():
            self.assertEquals((date, code), counter.parse_line_split(line))
