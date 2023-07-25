# import py_calendar
# from quantinv_common.timext import get_range_date
# print(py_calendar.date_range_rs("20230101","20240101"))
# print(get_range_date("20230101","20240101"))
import datetime
def datetime_range_date(start,end):
    start = datetime.date(int(start[0:4]),int(start[4:6]),int(start[6:8]))
    end = datetime.date(int(end[0:4]),int(end[4:6]),int(end[6:8]))
    return list(map(lambda date:date.strftime("%Y%m%d"),[datetime.date.fromordinal(x) for x in range(start.toordinal(),end.toordinal())]))


print(datetime_range_date("20150101","20250101"))



