#[doc = "Register `PRODUCT_ID0` reader"]
pub type R = crate::R<ProductId0Spec>;
#[doc = "Field `PRODUCT_ID0` reader - This one byte fixed code Identifies HDMI 's product line (\"A0h\" for DWC_hdmi_tx products)."]
pub type ProductId0R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - This one byte fixed code Identifies HDMI 's product line (\"A0h\" for DWC_hdmi_tx products)."]
    #[inline(always)]
    pub fn product_id0(&self) -> ProductId0R {
        ProductId0R::new(self.bits)
    }
}
#[doc = "This one byte fixed code Identifies HDMI 's product line (\"A0h\" for DWC_hdmi_tx products).\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`product_id0::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ProductId0Spec;
impl crate::RegisterSpec for ProductId0Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`product_id0::R`](R) reader structure"]
impl crate::Readable for ProductId0Spec {}
#[doc = "`reset()` method sets PRODUCT_ID0 to value 0xa0"]
impl crate::Resettable for ProductId0Spec {
    const RESET_VALUE: u8 = 0xa0;
}
