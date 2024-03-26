function submit(event) {
    document.getElementById('http-error').style.display = 'none';

    if (event) {
        event.preventDefault();
    }

    const latitude = document.querySelector('#latitude').value;
    const longitude = document.querySelector('#longitude').value;
    const range = document.querySelector('#range').value;

    if (!latitude || !longitude || !range) {
        return;
    }

    const url = location + `r/${latitude}/${longitude}/${range}`;

    fetch(url)
        .then(response => response.json())
        .then(response => response.states)
        .then(states =>
            states.map(s => ({
                ...s,
                speed: s.speed.toFixed(0),
                vertical_speed: s.vertical_speed.toFixed(1),
                altitude: s.altitude.toFixed(0),
                turn_rate: s.turn_rate.toFixed(1),
                course: s.course.toFixed(0),
                position: {
                    longitude: formatCoordinateValue(s.position.longitude, 'E', 'W'),
                    latitude: formatCoordinateValue(s.position.latitude, 'N', 'S'),
                }
            })
            ))
        .then(states =>
            Handlebars.templates.table({ states })
        )
        .then(html => {
            document.getElementById('table-container').innerHTML = html;
        })
        .catch(error => {
            document.getElementById('http-error').style.display = 'inherit';
            console.error(error);
        })
}

function formatCoordinateValue(latitude, directionPositive, directionNegative) {
    const degrees = Math.floor(latitude);
    const minutes = (latitude - degrees) * 60;
    const seconds = (minutes % 1) * 60;

    const direction = latitude > 0 ? directionPositive : directionNegative;

    return `${degrees.toString().padStart(3, '0')}°${minutes.toFixed(0)}'${seconds.toFixed(0)}" ${direction}`
}

function onClickWhatsAboveMe() {
    document.querySelector('#loading-position').style.display = 'inherit';

    navigator.geolocation.getCurrentPosition(
        (position) => {
            document.querySelector('#latitude').value = Math.round(position.coords.latitude * 1_000_000_000) / 1_000_000_000;
            document.querySelector('#longitude').value = Math.round(position.coords.longitude * 1_000_000_000) / 1_000_000_000;
            document.querySelector('#no-position-available').style.display = 'none';
            document.querySelector('#loading-position').style.display = 'none';
            submit();
        },
        () => {
            document.querySelector('#no-position-available').style.display = 'inherit';
            document.querySelector('#loading-position').style.display = 'none';
        });
}

document.querySelector('form').addEventListener('submit', submit);