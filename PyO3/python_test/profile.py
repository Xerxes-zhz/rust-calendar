# coding: utf-8
import py_calendar
from quantinv_common.timext import get_range_date
start = "20230101"
end = "20240101"
get_ipython().run_line_magic('timeit', 'py_calendar.date_range_rs(start,end)')
get_ipython().run_line_magic('timeit', 'get_range_date(start,end)')
end = "20500101"
get_ipython().run_line_magic('timeit', 'py_calendar.date_range_rs(start,end)')
get_ipython().run_line_magic('timeit', 'get_range_date(start,end)')
import pandas as pd
get_ipython().run_line_magic('timeit', 'pd.date_range(start,end,freq="D")')
get_ipython().run_line_magic('timeit', 'pd.date_range(start,end,freq="D").strftime("%Y%m%d")')
