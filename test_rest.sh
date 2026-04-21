#!/bin/bash
curl -s -X POST https://ai-prop-service.onrender.com/calculate_proportions \
-H "Content-Type: application/json" \
-d '{"overall": {"m": 10.0, "n": 20.0}, "group1": {"m": 5.0, "n": 10.0}, "group2": {"m": 5.0, "n": 10.0}}'
echo ""
