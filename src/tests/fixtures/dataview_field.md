---
title: Dataview field
---

# Dataview field

```
key:: value
[key:: value]
(key:: value)
```

```````````````````````````````` example
key:: value
.
<p><div class="dataview-field dataview-field-full-line">
  <span class="dataview-key">key</span>
  <span class="dataview-value">value</span>
</div></p>
````````````````````````````````

```````````````````````````````` example
[key:: value]
.
<p><div class="dataview-field dataview-field-bracket">
  <span class="dataview-key">key</span>
  <span class="dataview-value">value</span>
</div></p>
````````````````````````````````

```````````````````````````````` example
(key:: value)
.
<p><div class="dataview-field dataview-field-parenthesis">
  <span class="dataview-key">key</span>
  <span class="dataview-value">value</span>
</div></p>
````````````````````````````````

Style

```````````````````````````````` example
this is a **bold key**:: value
.
<p><div class="dataview-field dataview-field-full-line">
  <span class="dataview-key">this is a <strong>bold key</strong></span>
  <span class="dataview-value">value</span>
</div></p>
````````````````````````````````
