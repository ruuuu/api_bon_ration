<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>createOrderUsingPOST</name>
   <tag></tag>
   <elementGuidId>205a5398-b772-435d-a44c-49ed95e872f2</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;delivery_address\&quot;: {\n    \&quot;city\&quot;: \&quot;Астрахань\&quot;,\n    \&quot;entrance\&quot;: \&quot;string\&quot;,\n    \&quot;flat_number\&quot;: \&quot;67\&quot;,\n    \&quot;floor\&quot;: 4,\n    \&quot;house_number\&quot;: \&quot;67\&quot;,\n    \&quot;human_readable_address\&quot;: \&quot;string\&quot;,\n    \&quot;latitude\&quot;: 56,\n    \&quot;longitude\&quot;: 89,\n    \&quot;street\&quot;: \&quot;лоапрапорадр алжрлвра\&quot;\n  },\n  \&quot;delivery_time\&quot;: 25200000,\n  \&quot;first_day\&quot;: 1552028400000,\n  \&quot;number_of_days\&quot;: 3,\n  \&quot;payment_method\&quot;: \&quot;ONLINE\&quot;,\n \n  \&quot;ration_id\&quot;: 176,\n  \&quot;wishes\&quot;: \&quot;string\&quot;\n}&quot;,
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
      <value>Bearer eyJhbGciOiJIUzUxMiJ9.eyJjbGllbnRUeXBlIjoidXNlciIsInVzZXJfaWQiOjExLCJ0b2tlbl9leHBpcmF0aW9uX2RhdGUiOjE1NTM2MjkyNDM3NjQsInRva2VuX2NyZWF0ZV9kYXRlIjoxNTUxMjEwMDQzNzY0fQ.d2mju5IvzN4FgAbloPmpgLEY2q06E1qCYuokKUD29NkEf-A_Nab9JrlBUIYoeMKFmrNamvODW20Rhvsm1jOBgQ</value>
   </httpHeaderProperties>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>https://lifestyle-testapi.technaxis.com/v1/orders?</restUrl>
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
