<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>get_Promoter_Referral Info</name>
   <tag></tag>
   <elementGuidId>cf14569d-a891-4135-a7b8-4870f68b972c</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <httpBody></httpBody>
   <httpBodyContent></httpBodyContent>
   <httpBodyType></httpBodyType>
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
   <restRequestMethod>GET</restRequestMethod>
   <restUrl>https://lifestyle-testapi.technaxis.com/v1/superadmin/promoters/${promoterId}/referrals?page=0&amp;limit=10</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceFunction></soapServiceFunction>
   <variables>
      <defaultValue>'13'</defaultValue>
      <description></description>
      <id>8605cee1-e36f-448f-871f-23cb2ec2c915</id>
      <masked>false</masked>
      <name>promoterId</name>
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
