{%- if structure["header"] | length != 0 -%}
{{ structure["header"] -}}
{% endif -%}

{% if config.ifndef | length != 0 -%}
#ifndef {{ config.ifndef }}
#define {{ config.ifndef }}
{%- endif %}

{% if config.imports -%}
    {% for include in includes %}
#include {{ include }}
    {%- endfor -%}
{%- endif %}

{% if config.cpp_compat -%}
#ifdef __cplusplus
extern "C" {
#endif
{%- endif %}

{% for constant in constants %}
{{ constant -}}
{% endfor %}

{% for type in types %}
{{ type_name(name = type) -}}
{% endfor %}

{% for function in functions %}
{{ function -}}
{% endfor %}

{% if config.cpp_compat -%}
#ifdef __cplusplus
}
#endif
{%- endif %}

{{ structure.footer }}
