#[doc = "Register `P_REG_FRQ` reader"]
pub type R = crate::R<PRegFrqSpec>;
#[doc = "Register `P_REG_FRQ` writer"]
pub type W = crate::W<PRegFrqSpec>;
#[doc = "Field `FRQ` reader - digital output for debug, controlled by \n\npll_reg1&lt;7> and pll_reg4&lt;4> \n\nwhen pll_reg1&lt;7>=0 &amp; pll_reg4&lt;4>=0: \n\nhalf video clock frequency calculated by \n\nfrequency counter \n\nWhen pll_reg1&lt;7>=0 &amp; pll_reg4&lt;4>=1: \n\nhalf video clock frequency calculated \n\nfrq_vid_ck_in&lt;8:0> \n\nwhen pll_reg1&lt;7>=1: \n\nfrq&lt;1:0>: &lt;n_over, n_under> \n\nfrq&lt;3:2>: band&lt;1:0> \n\nfrq&lt;7:4>: 0"]
pub type FrqR = crate::FieldReader;
#[doc = "Field `FRQ` writer - digital output for debug, controlled by \n\npll_reg1&lt;7> and pll_reg4&lt;4> \n\nwhen pll_reg1&lt;7>=0 &amp; pll_reg4&lt;4>=0: \n\nhalf video clock frequency calculated by \n\nfrequency counter \n\nWhen pll_reg1&lt;7>=0 &amp; pll_reg4&lt;4>=1: \n\nhalf video clock frequency calculated \n\nfrq_vid_ck_in&lt;8:0> \n\nwhen pll_reg1&lt;7>=1: \n\nfrq&lt;1:0>: &lt;n_over, n_under> \n\nfrq&lt;3:2>: band&lt;1:0> \n\nfrq&lt;7:4>: 0"]
pub type FrqW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - digital output for debug, controlled by \n\npll_reg1&lt;7> and pll_reg4&lt;4> \n\nwhen pll_reg1&lt;7>=0 &amp; pll_reg4&lt;4>=0: \n\nhalf video clock frequency calculated by \n\nfrequency counter \n\nWhen pll_reg1&lt;7>=0 &amp; pll_reg4&lt;4>=1: \n\nhalf video clock frequency calculated \n\nfrq_vid_ck_in&lt;8:0> \n\nwhen pll_reg1&lt;7>=1: \n\nfrq&lt;1:0>: &lt;n_over, n_under> \n\nfrq&lt;3:2>: band&lt;1:0> \n\nfrq&lt;7:4>: 0"]
    #[inline(always)]
    pub fn frq(&self) -> FrqR {
        FrqR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - digital output for debug, controlled by \n\npll_reg1&lt;7> and pll_reg4&lt;4> \n\nwhen pll_reg1&lt;7>=0 &amp; pll_reg4&lt;4>=0: \n\nhalf video clock frequency calculated by \n\nfrequency counter \n\nWhen pll_reg1&lt;7>=0 &amp; pll_reg4&lt;4>=1: \n\nhalf video clock frequency calculated \n\nfrq_vid_ck_in&lt;8:0> \n\nwhen pll_reg1&lt;7>=1: \n\nfrq&lt;1:0>: &lt;n_over, n_under> \n\nfrq&lt;3:2>: band&lt;1:0> \n\nfrq&lt;7:4>: 0"]
    #[inline(always)]
    #[must_use]
    pub fn frq(&mut self) -> FrqW<PRegFrqSpec> {
        FrqW::new(self, 0)
    }
}
#[doc = "frequency counter ,digital output for debug\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`p_reg_frq::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`p_reg_frq::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PRegFrqSpec;
impl crate::RegisterSpec for PRegFrqSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`p_reg_frq::R`](R) reader structure"]
impl crate::Readable for PRegFrqSpec {}
#[doc = "`write(|w| ..)` method takes [`p_reg_frq::W`](W) writer structure"]
impl crate::Writable for PRegFrqSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets P_REG_FRQ to value 0"]
impl crate::Resettable for PRegFrqSpec {
    const RESET_VALUE: u32 = 0;
}
