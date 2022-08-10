import { check } from 'k6';
import http from 'k6/http';

export const options = {
    vus: 1000,
    duration: '1m',
    userAgent: 'alex/k6-test',
};

export default function () {
    const res = http.get('http://127.0.0.1:9123/img.php?v=2&id=eyJpY29uIjoiaWNvbi5wbmciLCJ1aWQiOjExMSwiY2lkIjoyMjIsIm9zIjoxMjMsImJyb3dzZXIiOjEyLCJjb3VudHJ5IjoyMTMsIm9wZXJhdG9yIjoxMjMsInN1YkFjYyI6MjMsInN1YklkIjoyMjIyMjIsImFkdlR5cGUiOjAsInRyYWZmaWNDaGFubmVsIjoyfQ==');

    check(res, {
        'status is 200': (r) => r.status === 200,
        'body is empty': (r) => r.body === '',
        'header X-Accel-Redirect is present': (r) => r.headers['X-Accel-Redirect'] === '/icon/111/icon.png',
    });
}
