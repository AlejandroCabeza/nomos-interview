# Ideas

## Possible Backends

- Trivia
    - https://opentdb.com/api_config.php

## Services

- DB
    - Store score (and other relevant info)

## Backends & Generics

- CLI Backends should have their own trait, and require repository (or rather, an adapter) to conform to it
    - ImageVisualizerBackend should handle any VisualizableImage
    - IdentifyQuestionsBackend should handle any Question with Image
    - etc.

- Abstract some Cli backend functionality into the trait.

## Formatter/Adapter

- One of the services should be an adapter that formats repository data for the Cli's Backend.
    - Or it could be just integrated into one of the services. Probably a middle service, tbf.

## Deduplicate entities

- Deduplicate serializers and entities
- Instead, use a trait both backends use as common points
