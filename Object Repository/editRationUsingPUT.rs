<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>editRationUsingPUT</name>
   <tag></tag>
   <elementGuidId>0c227fe7-f1d1-4e8d-8699-fb58eb78149f</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;calories\&quot;: 6043,\n  \&quot;description\&quot;: \&quot;kjfglksjlkgjsd dgkds\&quot;,\n  \n  \&quot;dishes\&quot;: [\n    {\n      \&quot;dish_ids\&quot;: [\n        24\n      ],\n      \&quot;mealtime\&quot;: \&quot;BREAKFAST\&quot;\n    }\n  ],\n  \&quot;for_bonus\&quot;: true,\n  \&quot;halal\&quot;: true,\n  \&quot;main_photo\&quot;: 66,\n  \&quot;name\&quot;: \&quot;jdfgjhs sdgshjd dghlad\&quot;,\n  \&quot;photo_ids\&quot;: [\n    99\n  ],\n  \&quot;price\&quot;: 1453,\n  \&quot;short_description\&quot;: \&quot;kdfjglkdjf glksjg sdjfhsdlg sdjghldsadgljad\&quot;,\n  \&quot;vegan\&quot;: true,\n  \&quot;vegetarian\&quot;: true\n}&quot;,
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
      <value>Bearer eyJhbGciOiJIUzUxMiJ9.eyJjbGllbnRUeXBlIjoidXNlciIsInVzZXJfaWQiOjE1LCJ0b2tlbl9leHBpcmF0aW9uX2RhdGUiOjE1NTM3Nzk5NjM5MjIsInRva2VuX2NyZWF0ZV9kYXRlIjoxNTUxMzYwNzYzOTIyfQ.ddB6q4IgMHaGPQMg8zEuoP2ZbhVTk_HU7rfXhv5W5RaWr-n0fZ5m1hAiaYmNsHJtG-tm6OwfJJtcCTsm7-vb5A</value>
   </httpHeaderProperties>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>PUT</restRequestMethod>
   <restUrl>https://lifestyle-testapi.technaxis.com/v1/admin/rations/${id}?</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceFunction></soapServiceFunction>
   <variables>
      <defaultValue>'158'</defaultValue>
      <description></description>
      <id>67481f8a-dcd5-427b-83e3-0b76e65ae53a</id>
      <masked>false</masked>
      <name>id</name>
   </variables>
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
