// Note that a dynamic `import` statement here is required due to
// webpack/webpack#6615, but in theory `import { greet } from './pkg';`
// will work here one day as well!
const rust = import('./pkg');

const gpx = "<?xml version='1.0' encoding='UTF-8'?>\
<gpx version=\"1.1\" creator=\"https://www.komoot.de\" xmlns=\"http://www.topografix.com/GPX/1/1\" xmlns:xsi=\"http://www.w3.org/2001/XMLSchema-instance\" xsi:schemaLocation=\"http://www.topografix.com/GPX/1/1 http://www.topografix.com/GPX/1/1/gpx.xsd\">\
  <metadata>\
    <name>Frascati-Pescasseroli 3</name>\
    <author>\
      <link href=\"https://www.komoot.de\">\
        <text>komoot</text>\
        <type>text/html</type>\
      </link>\
    </author>\
  </metadata>\
  <trk>\
    <name>Frascati-Pescasseroli 3</name>\
    <trkseg>\
      <trkpt lat=\"41.730051\" lon=\"13.757697\">\
        <ele>1000.642303</ele>\
        <time>2023-01-02T07:51:01.000Z</time>\
      </trkpt>\
      <trkpt lat=\"41.729984\" lon=\"13.757777\">\
        <ele>1000.642303</ele>\
        <time>2023-01-02T07:51:06.000Z</time>\
      </trkpt>\
      <trkpt lat=\"41.729970\" lon=\"13.757791\">\
        <ele>1000.642303</ele>\
        <time>2023-01-02T07:51:07.000Z</time>\
      </trkpt>\
      <trkpt lat=\"41.729822\" lon=\"13.757923\">\
        <ele>1000.642303</ele>\
        <time>2023-01-02T07:51:15.000Z</time>\
      </trkpt>\
      <trkpt lat=\"41.729683\" lon=\"13.758049\">\
        <ele>1000.642303</ele>\
        <time>2023-01-02T07:51:23.000Z</time>\
      </trkpt>\
      <trkpt lat=\"41.729598\" lon=\"13.758114\">\
        <ele>1000.642303</ele>\
        <time>2023-01-02T07:51:29.000Z</time>\
      </trkpt>\
      <trkpt lat=\"41.729596\" lon=\"13.758118\">\
        <ele>1000.642303</ele>\
        <time>2023-01-02T07:51:33.000Z</time>\
      </trkpt>\
      <trkpt lat=\"41.729576\" lon=\"13.758142\">\
        <ele>1000.642303</ele>\
        <time>2023-01-02T07:51:40.000Z</time>\
      </trkpt>\
      <trkpt lat=\"41.729517\" lon=\"13.758212\">\
        <ele>1000.642303</ele>\
        <time>2023-01-02T07:51:45.000Z</time>\
      </trkpt>\
      <trkpt lat=\"41.729429\" lon=\"13.758329\">\
        <ele>1000.642303</ele>\
        <time>2023-01-02T07:51:51.000Z</time>\
      </trkpt>\
    </trkseg>\
  </trk>\
</gpx>"

var map = L.map('map').setView([0, 0], 13);
L.tileLayer('https://tile.openstreetmap.org/{z}/{x}/{y}.png', {
  maxZoom: 19,
  attribution: '&copy; <a href=\"http://www.openstreetmap.org/copyright\">OpenStreetMap</a>'
}).addTo(map);

let o;
rust.then(m => {
  o = m.work2(gpx);
  map.setView([o[0].y, o[0].x], 13);
  var latLngPoints = [];
  o.forEach(point => {
    var latLng = L.latLng(point.y, point.x);
    console.log("Adding: ", point);
    latLngPoints.push(latLng)
  })
  L.polyline(latLngPoints, { color: 'red' }).addTo(map);
})


