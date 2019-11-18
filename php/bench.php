<?php

use Symfony\Component\DomCrawler\Crawler;

require __DIR__.'/vendor/autoload.php';

$count = 10 * 1000;
$count = 1000;
$html = file_get_contents('https://symfony.com/download');
$selector = 'h1';

print "FFI:\n";

$ffi = FFI::cdef(<<<EOH
const char *cssfilter(const char *html, const char *filter);
EOH, __DIR__.'/../target/release/libcssfilter.so');

echo " Found: ";
echo $ffi->cssfilter($html, $selector);
echo "\n";
$s = microtime(true);
for ($i = 0; $i < $count; ++$i) {
    $ffi->cssfilter($html, $selector);
}
printf(" Duration: %.3fs\n\n", microtime(true) - $s);

print "symfony/crawler:\n";

echo " Found: ";
echo (new Crawler($html))->filter($selector)->text();
echo "\n";
$s = microtime(true);
for ($i = 0; $i < $count; ++$i) {
    (new Crawler($html))->filter($selector)->text();

}
printf(" Duration: %.3fs\n\n", microtime(true) - $s);

echo 'Same Result:' . ($ffi->cssfilter($html, $selector) === (new Crawler($html))->filter($selector)->text() ? "✅" : "❌");
echo "\n";
