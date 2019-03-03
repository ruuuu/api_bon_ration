<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>updateInfoUsingPUT</name>
   <tag></tag>
   <elementGuidId>d73e1712-f5d8-40b6-9631-37e44e21276f</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;about\&quot;: \&quot;kjslkgjsdg sdkgksjdlgksdg sdgj;dg dgja;ga gadkgad;g\&quot;,\n  \&quot;address\&quot;: \&quot;Kazan\&quot;,\n  \&quot;closing_time\&quot;: 43200000,\n  \n  \&quot;food_for_points\&quot;: true,\n  \&quot;logo_id\&quot;: 66,\n  \n  \&quot;morning_delivery_time_from_ms\&quot;: 18000000,\n  \&quot;morning_delivery_time_to_ms\&quot;: 25200000,\n  \&quot;name\&quot;: \&quot;Компания Life\&quot;,\n  \&quot;opening_time\&quot;: 25200000,\n  \&quot;payment_types\&quot;: [\n    \&quot;ONLINE\&quot;\n  ],\n  \&quot;phone\&quot;: \&quot;87654345098\&quot;,\n  \&quot;photo_id\&quot;: 66\n}&quot;,
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
   <restUrl>https://lifestyle-testapi.technaxis.com/v1/admin/companies/${companyId}?</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceFunction></soapServiceFunction>
   <variables>
      <defaultValue>'8'</defaultValue>
      <description></description>
      <id>7a780029-fb71-4ab8-a8b8-7115bfce992b</id>
      <masked>false</masked>
      <name>companyId</name>
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
