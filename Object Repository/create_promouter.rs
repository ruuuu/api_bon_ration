<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>create_promouter</name>
   <tag></tag>
   <elementGuidId>896b2572-6315-49d9-9f3d-41f14b6a8ed8</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;full_name\&quot;: \&quot;kdfjglsj sdgksdgjlds\&quot;,\n  \&quot;login\&quot;: \&quot;ghdhdfhdfh@mail.ru\&quot;,\n  \&quot;organization\&quot;: \&quot;string\&quot;,\n  \&quot;percentage_from_orders\&quot;: 3,\n  \&quot;phone\&quot;: \&quot;8976543216\&quot;,\n  \&quot;position\&quot;: \&quot;hgjfj fghdghdhdfh s\&quot;,\n  \&quot;promo_code\&quot;: \&quot;5675ryryrwyw\&quot;\n}&quot;,
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
      <name>Authorization</name>
      <type>Main</type>
      <value>Bearer eyJhbGciOiJIUzUxMiJ9.eyJjbGllbnRUeXBlIjoidXNlciIsInVzZXJfaWQiOjMxLCJ0b2tlbl9leHBpcmF0aW9uX2RhdGUiOjE1NTM1MDc4MDY3NDEsInRva2VuX2NyZWF0ZV9kYXRlIjoxNTUxMDg4NjA2NzQxfQ.fXJ6-TLwFSdMtNmsHHGSlmLuQpeQO6JVzcpzOqJREwZ4aRZ2qHgczNhORq8ftuNf9ZowLYc0SChSZYhT8yToUw</value>
   </httpHeaderProperties>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>https://lifestyle-testapi.technaxis.com/v1/superadmin/promoters?</restUrl>
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
