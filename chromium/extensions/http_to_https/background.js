'use strict';
if (chrome.declarativeWebRequest) {
    chrome.runtime.onInstalled.addListener(setupDWR);
} else {
    chrome.webRequest.onBeforeRequest.addListener(function(details) {
        return {
            redirectUrl: details.url.replace('http:', 'https:')
        };
    }, {
        urls: ['http://*/*'],
    }, ['blocking']);
}

function setupDWR() {
    var rule2 = {
        conditions: [
            new chrome.declarativeWebRequest.RequestMatcher({
                url: {
                    schemes: ['http'],
                },
                stages: ['onBeforeRequest']
            }),
        ],
        actions: [
            new chrome.declarativeWebRequest.RedirectByRegEx({
                from: 'http:(.*)',
                to: 'https:$1',
            })
        ],
        id: 'http-to-https',
    };

    chrome.declarativeWebRequest.onRequest.removeRules([rule2.id], function() {
        chrome.declarativeWebRequest.onRequest.addRules([rule2], function() {
            console.log('Will redirect all http to https');
        });
    });
}
