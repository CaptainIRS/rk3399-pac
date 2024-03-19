#[doc = "Register `DDR_DENALI_CTL_31` reader"]
pub type R = crate::R<DdrDenaliCtl31Spec>;
#[doc = "Register `DDR_DENALI_CTL_31` writer"]
pub type W = crate::W<DdrDenaliCtl31Spec>;
#[doc = "Field `TRP_F2` reader - DRAM TRP value for frequency copy 2 in cycles."]
pub type TrpF2R = crate::FieldReader;
#[doc = "Field `TRP_F2` writer - DRAM TRP value for frequency copy 2 in cycles."]
pub type TrpF2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `TFAW_F2` reader - DRAM TFAW value for frequency copy 2 in cycles."]
pub type TfawF2R = crate::FieldReader;
#[doc = "Field `TFAW_F2` writer - DRAM TFAW value for frequency copy 2 in cycles."]
pub type TfawF2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `CA_DEFAULT_VAL_F2` reader - Defines how unused address/ command bits are driven for frequency copy 2. Set to 1 to use last value or clear to 0 to drive low."]
pub type CaDefaultValF2R = crate::BitReader;
#[doc = "Field `CA_DEFAULT_VAL_F2` writer - Defines how unused address/ command bits are driven for frequency copy 2. Set to 1 to use last value or clear to 0 to drive low."]
pub type CaDefaultValF2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRTP_F0` reader - DRAM TRTP value for frequency copy 0 in cycles."]
pub type TrtpF0R = crate::FieldReader;
#[doc = "Field `TRTP_F0` writer - DRAM TRTP value for frequency copy 0 in cycles."]
pub type TrtpF0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - DRAM TRP value for frequency copy 2 in cycles."]
    #[inline(always)]
    pub fn trp_f2(&self) -> TrpF2R {
        TrpF2R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - DRAM TFAW value for frequency copy 2 in cycles."]
    #[inline(always)]
    pub fn tfaw_f2(&self) -> TfawF2R {
        TfawF2R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 16 - Defines how unused address/ command bits are driven for frequency copy 2. Set to 1 to use last value or clear to 0 to drive low."]
    #[inline(always)]
    pub fn ca_default_val_f2(&self) -> CaDefaultValF2R {
        CaDefaultValF2R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 24:31 - DRAM TRTP value for frequency copy 0 in cycles."]
    #[inline(always)]
    pub fn trtp_f0(&self) -> TrtpF0R {
        TrtpF0R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - DRAM TRP value for frequency copy 2 in cycles."]
    #[inline(always)]
    #[must_use]
    pub fn trp_f2(&mut self) -> TrpF2W<DdrDenaliCtl31Spec> {
        TrpF2W::new(self, 0)
    }
    #[doc = "Bits 8:15 - DRAM TFAW value for frequency copy 2 in cycles."]
    #[inline(always)]
    #[must_use]
    pub fn tfaw_f2(&mut self) -> TfawF2W<DdrDenaliCtl31Spec> {
        TfawF2W::new(self, 8)
    }
    #[doc = "Bit 16 - Defines how unused address/ command bits are driven for frequency copy 2. Set to 1 to use last value or clear to 0 to drive low."]
    #[inline(always)]
    #[must_use]
    pub fn ca_default_val_f2(&mut self) -> CaDefaultValF2W<DdrDenaliCtl31Spec> {
        CaDefaultValF2W::new(self, 16)
    }
    #[doc = "Bits 24:31 - DRAM TRTP value for frequency copy 0 in cycles."]
    #[inline(always)]
    #[must_use]
    pub fn trtp_f0(&mut self) -> TrtpF0W<DdrDenaliCtl31Spec> {
        TrtpF0W::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_ctl_31::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_denali_ctl_31::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliCtl31Spec;
impl crate::RegisterSpec for DdrDenaliCtl31Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_ctl_31::R`](R) reader structure"]
impl crate::Readable for DdrDenaliCtl31Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_denali_ctl_31::W`](W) writer structure"]
impl crate::Writable for DdrDenaliCtl31Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_DENALI_CTL_31 to value 0"]
impl crate::Resettable for DdrDenaliCtl31Spec {
    const RESET_VALUE: u32 = 0;
}
