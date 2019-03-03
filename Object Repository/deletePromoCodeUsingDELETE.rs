<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>deletePromoCodeUsingDELETE</name>
   <tag></tag>
   <elementGuidId>2ba6afc1-f768-4fb5-8507-3a43456c4bb3</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;&quot;,
  &quot;contentType&quot;: &quot;text/plain&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
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
      <value>Bearer eyJhbGciOiJIUzUxMiJ9.eyJjbGllbnRUeXBlIjoidXNlciIsInVzZXJfaWQiOjMxLCJ0b2tlbl9leHBpcmF0aW9uX2RhdGUiOjE1NTM1MDc4MDY3NDEsInRva2VuX2NyZWF0ZV9kYXRlIjoxNTUxMDg4NjA2NzQxfQ.fXJ6-TLwFSdMtNmsHHGSlmLuQpeQO6JVzcpzOqJREwZ4aRZ2qHgczNhORq8ftuNf9ZowLYc0SChSZYhT8yToUw</value>
   </httpHeaderProperties>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>DELETE</restRequestMethod>
   <restUrl>https://lifestyle-testapi.technaxis.com/v1/superadmin/promo/${promoCodeId}?</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceFunction></soapServiceFunction>
   <variables>
      <defaultValue>'9'</defaultValue>
      <description></description>
      <id>145d4862-b469-4104-b831-189ed2adb2a1</id>
      <masked>false</masked>
      <name>promoCodeId</name>
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
