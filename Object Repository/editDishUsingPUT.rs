<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>editDishUsingPUT</name>
   <tag></tag>
   <elementGuidId>c25f1b2b-ed5d-455e-a281-9c47177baf94</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;calories\&quot;: 649,\n  \&quot;description\&quot;: \&quot;fjgdg fsg;sgsdg slgksdgsgkssdkg;sgd\&quot;,\n  \&quot;name\&quot;: \&quot;названеи блюда\&quot;,\n  \&quot;photo_id\&quot;: 738,\n  \&quot;weight\&quot;: 4332\n}&quot;,
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
   <restUrl>https://lifestyle-testapi.technaxis.com/v1/admin/dishes/${id}?</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceFunction></soapServiceFunction>
   <variables>
      <defaultValue>'24'</defaultValue>
      <description></description>
      <id>69f8c9ce-045e-491f-8cd7-1f5b5a1e0d98</id>
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
