
import glob
import re
import logging
import datetime
from pprint import pprint  # noqa
from collections import defaultdict


logging.basicConfig(level=logging.DEBUG, format='%(asctime)s %(message)s')
def_logger = logging.getLogger(__name__)


class CodesByDateTimeCounter:
    def __init__(self):
        # self.common_regex = re.compile(r'^\d{1,3}\.\d{1,3}\.\d{1,3}.\d{1,3} [^ ]+ [^ ]+ \[(?P<datetime>[^\]]+):\d{2} \+\d{4}\] "[^"]+" (?P<code>\d{3})')  # noqa
        self.common_regex = re.compile(r'^[^\[]+\[(?P<datetime>[^\]]+):\d{2} [-+]\d{4}\] "[^"]+" (?P<code>\d{3})')  # noqa
        self.seen_codes = {}
        self.seen_dates = {}
        self.codes_by_time = defaultdict(lambda: defaultdict(int))

    def add_line(self, line):
        dt, code = self.parse_line_regex(line)
        if dt is None or code is None:
            return False
        else:
            self.codes_by_time[dt][code] += 1
            self.seen_codes[code] = True
            self.seen_dates[dt] = True
            return True

    def parse_line_scan(self, line):
        datestart = line.index('[') + 1
        dateend = line.index(']', datestart)
        minuteend = line.rindex(':', datestart, dateend)

        rqlinestart = line.index('"') + 1
        rqlineend = line.index('"', rqlinestart) + 1

        return line[datestart:minuteend], line[(rqlineend + 1):(rqlineend + 4)]

    def parse_line_split(self, line):
        _, dateend = line.split('[', 2)
        datestart, requestline, codes, _ = dateend.split('"', 3)
        _, status, _ = codes.split(' ', 2)

        dt, _ = datestart.rsplit(':', 1)

        return dt, status

    def parse_line_regex(self, line):
        match = self.common_regex.match(line)
        if match is None:
            return None, None
        return match.group('datetime'), match.group('code')

    def write_csv(self, fp):
        seen_codes = sorted(self.seen_codes)
        seen_dates = sorted(self.seen_dates,
                            key=lambda d: datetime.datetime.strptime(
                                d, '%d/%b/%Y:%H:%M').timestamp())

        headers = ['time'] + seen_codes

        fp.write(', '.join(headers) + "\n")
        for dt in seen_dates:
            fp.write(dt)

            for code in seen_codes:
                fp.write(", " + str(self.codes_by_time[dt][code]))
            fp.write("\n")


def parse_file(logfile, logger=def_logger, codecounter=None):
    parsed_lines = 0
    if codecounter is None:
        codecounter = CodesByDateTimeCounter()
    logger.info("Starting %s", logfile)
    for line in open(logfile):
        if not codecounter.add_line(line):
            logger.warn("Line did not parse: '%s'", line)
        parsed_lines += 1
        if parsed_lines % 1000000 == 0:
            logger.info("Parsed %d lines", parsed_lines)
    logger.info("Finished with %s at %d lines", logfile, parsed_lines)


if __name__ == "__main__":
    files = glob.glob('./example-logfile-*')
    codecounter = CodesByDateTimeCounter()
    for fname in files:
        parse_file(fname, def_logger, codecounter)

    with open('./out.csv', 'w') as fp:
        codecounter.write_csv(fp)
