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