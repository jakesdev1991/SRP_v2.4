package srp.policy.sbom
default allow = false

# Deny if any component is missing a SLSA provenance level.
allow {
    count(missing_slsa) == 0
    count(failed_slsa_level) == 0
    count(disallowed_licenses) == 0
}

missing_slsa[component] {
    component := input.components[_]
    not component.slsa_provenance
}

failed_slsa_level[component] {
    component := input.components[_]
    component.slsa_provenance.level < 3
}

disallowed_licenses[component] {
    component := input.components[_]
    component.license == "GPL-3.0"
}
