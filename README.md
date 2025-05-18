# apache-echarts-wrapper-rust
Rust bindings to [Apache ECharts](https://echarts.apache.org)

## Why?
I want to render HTML for charts server-side in my Rust app.

## How?
Create options with 
```
 let chart = EChartsOption::<f64, &str>::new()
```
Then populate it with chart data. You can add more than one.
```
 chart.add_series(
        SeriesType::Line,
        "first_set",
        vec![(12.5,"First"),(14.0,"Second"),(15.0,"Third"),(10.0,"Fourth")]
      )
      .add_series(
        SeriesType::Line,
        "second_set",
        vec![(2.0,"First"),(14.0,"Third"),(15.0,"Third"),(20.0,"First")]
      )
```

Build it.
```
 let template =  chart.build(Size::pixels(600),Size::pixels(400));
```

Size can be expressed in pixels or percents.
Build command returns a [sailfish](https://github.com/rust-sailfish/sailfish) 
template that can be converted to string or written to buffer.
This results in HTML that consists of div tag and script with chart pointing to it,
that you can directly put inside your page via template, 
or return via htmx.
The page you are inserting this into should have echarts and ecStat imported, 
meaning your header tag should contain something like following.
```
<script src="https://cdn.jsdelivr.net/npm/echarts@5.6.0/dist/echarts.min.js"></script>
<script src="https://cdn.jsdelivr.net/npm/echarts-stat@1.2.0/dist/ecStat.min.js"></script>
```
If that is not enough for your needs, and you need more control, you can just directly serialize
EChartsOption object into JSON using [serde](https://serde.rs/).
It will result in a valid [option](https://echarts.apache.org/en/option.html#title) object.

For more examples see tests in the lib.rs file.