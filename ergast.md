# Documentation
The Ergast Developer API is an experimental web service which provides a historical record of motor racing data for non-commercial purposes. Please read the terms and conditions of use. The API provides data for the Formula One series, from the beginning of the world championships in 1950.
<
Source code for a wide range of projects using the API can be found on the Ergast API Topic Page on GitHub. Example applications are showcased in the Application Gallery.

Non-programmers can query the database using the manual interface or download the database tables in CSV format for import into spreadsheets or analysis software.

If you have any comments or suggestions please post them on the Feedback page. If you find any bugs or errors in the data please report them on the Bug Reports page. Any enhancements to the API will be reported on the News page.

# Overview

All API queries require a GET request using a URL of the form:
```
http[s]://ergast.com/api/<series>/<season>/<round>/...

where:
<series> 	should be set to “f1”
<season> 	is a 4 digit integer
<round> 	is a 1 or 2 digit integer
```
For queries concerning a whole season, or final standings, the round element may be omitted. For example:

http://ergast.com/api/f1/2008/...

For queries concerning the whole series both the round and the season elements may be omitted. For example:

http://ergast.com/api/f1/...

To specify the current season the <season> field may be set to “current”. To specify the previous or next race within a season the <round> field may be set to “last” or “next” respectively. For example:

http://ergast.com/api/f1/2005/last/...
http://ergast.com/api/f1/current/next/...

For more information about these shortcuts see the FAQ.

The type of information returned by a query is determined by the remainder of the URL. The following options are available:
Season List 	Race Schedule 	Race Results
Qualifying Results 	Standings 	Driver Information
Constructor Information 	Circuit Information 	Finishing Status
Lap Times 	Pit Stops 	

You can generate valid URLs for the API using the manual interface.

# Response formats

The API supports XML, JSON and JSONP response formats. XML is returned by default or when “.xml” is appended to URLs. JSON is obtained by appending “.json” to URLs. JSONP is obtained by appending “.json” and a query parameter named “callback” which must specify a valid Javascript function name or object property function reference. For example:

http://ergast.com/api/f1/drivers.json?callback=myParser

XML responses conform to the Ergast Motor Racing XML Schema.
Response paging

The number of results that are returned can be controlled using a limit query parameter, up to a maximum value of 1000. Please use the smallest value that your application needs. If not specified, the default value is 30.

An offset into the result set can also be specified using an offset query parameter. If not specified the default offset is zero. For example, the following URL returns the third page of driver information containing ten entries per page:

http://ergast.com/api/f1/drivers?limit=10&offset=20

The total number of available results is indicated by a total attribute in the root element of the response.

# Caching

To improve the performance of your applications and to minimise the load on the API server please implement caching, either on your server and/or in your applications.
Database Image and Runtime