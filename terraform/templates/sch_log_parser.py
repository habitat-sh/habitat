import time
from datetime import datetime

def my_log_parser(logger, line):
    if line.count(',') >= 6:
        date, report_type, group_id, job_id, event, package, rest = line.split(',',6)

        if report_type == 'J' and event != 'Pending':
            date = datetime.strptime(date, "%Y-%m-%d %H:%M:%S")
            date = time.mktime(date.timetuple())
            url = '${bldr_url}/#/pkgs/{0}/builds/{1}'.format(package, job_id)

            if event == 'Failed':
                error = rest.split(',')[-1]
                message = package + ' ' + error + ' ' + url
            elif event == 'Complete':
                message = package + ' ' + url
            else:
                message = package + ' grp:' + group_id + ' job:' + job_id

            logged_event = {
                'msg_title': event,
                'timestamp': date,
                'msg_text': message,
                'priority': 'normal',
                'event_type': report_type,
                'aggregation_key': group_id,
                'alert_type': 'info'
            }
            return logged_event

    return None
