#[doc = "Register `PRODUCT_ID1` reader"]
pub type R = crate::R<ProductId1Spec>;
#[doc = "Field `PRODUCT_ID1_TX` reader - This bit Identifies H Controller according to HDMI\n\nproduct line."]
pub type ProductId1TxR = crate::BitReader;
#[doc = "Field `PRODUCT_ID1_RX` reader - This bit Identifies HDMI 's DWC_hdmi_rx Controller\n\naccording to HDMI product line."]
pub type ProductId1RxR = crate::BitReader;
#[doc = "Field `PRODUCT_ID1_HDCP` reader - These bits identify a HDMI Controller with HDCP\n\nencryption according to HDMI product line.\n\nReset Value: (HDCP== 1) ? 3 : 0"]
pub type ProductId1HdcpR = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - This bit Identifies H Controller according to HDMI\n\nproduct line."]
    #[inline(always)]
    pub fn product_id1_tx(&self) -> ProductId1TxR {
        ProductId1TxR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - This bit Identifies HDMI 's DWC_hdmi_rx Controller\n\naccording to HDMI product line."]
    #[inline(always)]
    pub fn product_id1_rx(&self) -> ProductId1RxR {
        ProductId1RxR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 6:7 - These bits identify a HDMI Controller with HDCP\n\nencryption according to HDMI product line.\n\nReset Value: (HDCP== 1) ? 3 : 0"]
    #[inline(always)]
    pub fn product_id1_hdcp(&self) -> ProductId1HdcpR {
        ProductId1HdcpR::new((self.bits >> 6) & 3)
    }
}
#[doc = "Product Identification Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`product_id1::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ProductId1Spec;
impl crate::RegisterSpec for ProductId1Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`product_id1::R`](R) reader structure"]
impl crate::Readable for ProductId1Spec {}
#[doc = "`reset()` method sets PRODUCT_ID1 to value 0x01"]
impl crate::Resettable for ProductId1Spec {
    const RESET_VALUE: u8 = 0x01;
}
