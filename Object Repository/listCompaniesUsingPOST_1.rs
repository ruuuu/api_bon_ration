<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>listCompaniesUsingPOST_1</name>
   <tag></tag>
   <elementGuidId>2545503a-b126-4668-89e8-cf717dec0add</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;city_id\&quot;: 1,\n  \&quot;companies_sort_type\&quot;: \&quot;BY_RATING\&quot;,\n  \&quot;delivery_time_from_ms\&quot;: 3600000,\n  \&quot;delivery_time_to_ms\&quot;: 32400000,\n  \&quot;food_for_points\&quot;: true,\n  \&quot;halal_table\&quot;: true,\n  \&quot;limit\&quot;: 10,\n  \&quot;max_ration_calories\&quot;: 1456,\n  \&quot;max_ration_price\&quot;: 1789,\n  \&quot;min_ration_calories\&quot;: 345,\n  \&quot;min_ration_price\&quot;: 45,\n  \&quot;page\&quot;: 1,\n  \n  \&quot;receive_orders_tomorrow\&quot;: true,\n  \&quot;vegan_table\&quot;: true,\n  \&quot;vegetarian_table\&quot;: true\n}&quot;,
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
   <restUrl>https://lifestyle-testapi.technaxis.com/v1/companies/list?</restUrl>
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
