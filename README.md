# Idea

According to information from http://ergast.com/mrd/, Ergast is scheduled to cease operations at the end of the 2024 F1 season next year. This new initiative is intended to serve as a potential replacement.

# <b>API REFERENCE IS SUBJECT TO CHANGE<b>


# API

# Options

- [Seasons](#seasons-options)
- Race schedules
- Qualifying results
- Race Results
- Standings
- Driver information
- Constructors information
- Circuit information
- Finishing status
- Lap Times
- Pit Stops


## Seasons (Options)

```html
- /circuits/<circuitId>
- /constructors/<constructorId>
- /drivers/<driverId>
- /grid/<position>
- /results/<position>
- /fastest/<rank>
- /status/<statusId>
```

```
/api/v1/f1/drivers/alonso/constructors/renault/seasons

/api/v1/f1/drivers/alonso/driverStandings/1/seasons

/api/v1/f1/constructors/renault/constructorStandings/1/seasons
```

## Qualifying (Options)

```html
/circuits/<circuitId>
/constructors/<constructorId>
/drivers/<driverId>
/grid/<position>
/results/<position>
/fastest/<rank>
/status/<statusId>
```

```
api/v1/f1/2008/drivers/

api/v1/f1/drivers/alonso/constructors/renault/qualifying

api/v1/f1/2008/qualifying/1
```


### How it should work

```
/api/v1/f1/<data>
```