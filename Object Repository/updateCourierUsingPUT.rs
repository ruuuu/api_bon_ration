<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>updateCourierUsingPUT</name>
   <tag></tag>
   <elementGuidId>25349780-9513-4f8e-bbf1-748200564561</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;address\&quot;: {\n    \&quot;city\&quot;: \&quot;kfjgflkjg fgksdj\&quot;,\n    \&quot;entrance\&quot;: \&quot;5\&quot;,\n    \&quot;flat_number\&quot;: \&quot;4\&quot;,\n    \&quot;floor\&quot;: 3,\n    \&quot;house_number\&quot;: \&quot;34a\&quot;,\n    \&quot;human_readable_address\&quot;: \&quot;string kdfjkgljdflg sgsdggdsg\&quot;,\n    \&quot;latitude\&quot;: -90,\n    \&quot;longitude\&quot;: 34,\n    \&quot;street\&quot;: \&quot;kxfjglkg sdkgjsdkgds glsdjg\&quot;\n  },\n  \&quot;email_enabled\&quot;: true,\n  \&quot;first_name\&quot;: \&quot;Rufina\&quot;,\n  \&quot;image_profile_id\&quot;: 480,\n  \&quot;phone\&quot;: \&quot;9878989876\&quot;,\n  \&quot;pushes_enabled\&quot;: false\n}&quot;,
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
      <value>Bearer eyJhbGciOiJIUzUxMiJ9.eyJjbGllbnRUeXBlIjoidXNlciIsInVzZXJfaWQiOjMxLCJ0b2tlbl9leHBpcmF0aW9uX2RhdGUiOjE1NTM1MDc4MDY3NDEsInRva2VuX2NyZWF0ZV9kYXRlIjoxNTUxMDg4NjA2NzQxfQ.fXJ6-TLwFSdMtNmsHHGSlmLuQpeQO6JVzcpzOqJREwZ4aRZ2qHgczNhORq8ftuNf9ZowLYc0SChSZYhT8yToUw</value>
   </httpHeaderProperties>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>PUT</restRequestMethod>
   <restUrl>https://lifestyle-testapi.technaxis.com/v1/admin/couriers/${courierId}?</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceFunction></soapServiceFunction>
   <variables>
      <defaultValue>'18'</defaultValue>
      <description></description>
      <id>afbf6e51-bf5a-451a-80de-dc26f2be1f53</id>
      <masked>false</masked>
      <name>courierId</name>
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
