#[doc = "Register `ATE_TEST_CTL` reader"]
pub type R = crate::R<AteTestCtlSpec>;
#[doc = "Register `ATE_TEST_CTL` writer"]
pub type W = crate::W<AteTestCtlSpec>;
#[doc = "Field `ATE_CLR_ERR` reader - Clear error counter \n\n\\[3\\]:lane3,\\[2\\]:lane2,\\[1\\]:lane1,\\[0\\]:lane0"]
pub type AteClrErrR = crate::FieldReader;
#[doc = "Field `ATE_CLR_ERR` writer - Clear error counter \n\n\\[3\\]:lane3,\\[2\\]:lane2,\\[1\\]:lane1,\\[0\\]:lane0"]
pub type AteClrErrW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ATE_ERR_GEN_EN_IN` reader - Insert a ERR for PHY ATE test. Self clear"]
pub type AteErrGenEnInR = crate::BitReader;
#[doc = "Field `ATE_ERR_GEN_EN_IN` writer - Insert a ERR for PHY ATE test. Self clear"]
pub type AteErrGenEnInW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `ATE_TEST_DATA_INV` reader - Invert ate test data"]
pub type AteTestDataInvR = crate::BitReader;
#[doc = "Field `ATE_TEST_DATA_INV` writer - Invert ate test data"]
pub type AteTestDataInvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_ATE` reader - ATE test enable \n\nBit 15~12: Reserved for analog test. \n\nBit 11: ate_en in ch3, \n\nBit 10: ate_en in ch2, \n\nBit 9: ate_en in ch1 \n\nBit 8: ate_en in ch0"]
pub type TxAteR = crate::FieldReader;
#[doc = "Field `TX_ATE` writer - ATE test enable \n\nBit 15~12: Reserved for analog test. \n\nBit 11: ate_en in ch3, \n\nBit 10: ate_en in ch2, \n\nBit 9: ate_en in ch1 \n\nBit 8: ate_en in ch0"]
pub type TxAteW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:3 - Clear error counter \n\n\\[3\\]:lane3,\\[2\\]:lane2,\\[1\\]:lane1,\\[0\\]:lane0"]
    #[inline(always)]
    pub fn ate_clr_err(&self) -> AteClrErrR {
        AteClrErrR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - Insert a ERR for PHY ATE test. Self clear"]
    #[inline(always)]
    pub fn ate_err_gen_en_in(&self) -> AteErrGenEnInR {
        AteErrGenEnInR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Invert ate test data"]
    #[inline(always)]
    pub fn ate_test_data_inv(&self) -> AteTestDataInvR {
        AteTestDataInvR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 8:15 - ATE test enable \n\nBit 15~12: Reserved for analog test. \n\nBit 11: ate_en in ch3, \n\nBit 10: ate_en in ch2, \n\nBit 9: ate_en in ch1 \n\nBit 8: ate_en in ch0"]
    #[inline(always)]
    pub fn tx_ate(&self) -> TxAteR {
        TxAteR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Clear error counter \n\n\\[3\\]:lane3,\\[2\\]:lane2,\\[1\\]:lane1,\\[0\\]:lane0"]
    #[inline(always)]
    #[must_use]
    pub fn ate_clr_err(&mut self) -> AteClrErrW<AteTestCtlSpec> {
        AteClrErrW::new(self, 0)
    }
    #[doc = "Bit 4 - Insert a ERR for PHY ATE test. Self clear"]
    #[inline(always)]
    #[must_use]
    pub fn ate_err_gen_en_in(&mut self) -> AteErrGenEnInW<AteTestCtlSpec> {
        AteErrGenEnInW::new(self, 4)
    }
    #[doc = "Bit 5 - Invert ate test data"]
    #[inline(always)]
    #[must_use]
    pub fn ate_test_data_inv(&mut self) -> AteTestDataInvW<AteTestCtlSpec> {
        AteTestDataInvW::new(self, 5)
    }
    #[doc = "Bits 8:15 - ATE test enable \n\nBit 15~12: Reserved for analog test. \n\nBit 11: ate_en in ch3, \n\nBit 10: ate_en in ch2, \n\nBit 9: ate_en in ch1 \n\nBit 8: ate_en in ch0"]
    #[inline(always)]
    #[must_use]
    pub fn tx_ate(&mut self) -> TxAteW<AteTestCtlSpec> {
        TxAteW::new(self, 8)
    }
}
#[doc = "ATE test control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ate_test_ctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ate_test_ctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AteTestCtlSpec;
impl crate::RegisterSpec for AteTestCtlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ate_test_ctl::R`](R) reader structure"]
impl crate::Readable for AteTestCtlSpec {}
#[doc = "`write(|w| ..)` method takes [`ate_test_ctl::W`](W) writer structure"]
impl crate::Writable for AteTestCtlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x1f;
}
#[doc = "`reset()` method sets ATE_TEST_CTL to value 0"]
impl crate::Resettable for AteTestCtlSpec {
    const RESET_VALUE: u32 = 0;
}
