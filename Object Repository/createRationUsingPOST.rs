<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>createRationUsingPOST</name>
   <tag></tag>
   <elementGuidId>e72ecd20-8ae8-4763-90b2-78356452adba</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;calories\&quot;: 7543,\n  \&quot;description\&quot;: \&quot;описание рациона компании\&quot;,\n  \&quot;discounts\&quot;: [\n    {\n      \&quot;discount_price\&quot;: 90,\n      \&quot;min_days_amount\&quot;: 2\n    }\n  ],\n  \&quot;dishes\&quot;: [\n    {\n      \&quot;dish_ids\&quot;: [\n        24\n      ],\n      \&quot;mealtime\&quot;: \&quot;BREAKFAST\&quot;\n    }\n  ],\n  \&quot;for_bonus\&quot;: true,\n  \&quot;halal\&quot;: true,\n  \&quot;main_photo\&quot;: 68,\n  \&quot;name\&quot;: \&quot;название рациона\&quot;,\n  \&quot;photo_ids\&quot;: [\n    68\n  ],\n  \&quot;price\&quot;: 1990,\n  \&quot;short_description\&quot;: \&quot;string\&quot;,\n  \&quot;trial_price\&quot;: 60,\n  \&quot;vegan\&quot;: true,\n  \&quot;vegetarian\&quot;: true\n}&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Accept</name>
      <type>Main</type>
      <value>application/json</value>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Bearer eyJhbGciOiJIUzUxMiJ9.eyJjbGllbnRUeXBlIjoidXNlciIsInVzZXJfaWQiOjE1LCJ0b2tlbl9leHBpcmF0aW9uX2RhdGUiOjE1NTM2MTA4NDcyMTgsInRva2VuX2NyZWF0ZV9kYXRlIjoxNTUxMTkxNjQ3MjE4fQ.qL7x1KYsQQkwogU-JVUdcjdJRhbPEN13B2nQoWxL1DEdEPtxlzPjE-NHdQFgnFCOje_K5Xy0JM1uQCwT418zEw</value>
   </httpHeaderProperties>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>https://lifestyle-testapi.technaxis.com/v1/admin/rations/new?</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceFunction></soapServiceFunction>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()
</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
