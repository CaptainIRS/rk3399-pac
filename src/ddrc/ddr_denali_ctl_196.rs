#[doc = "Register `DDR_DENALI_CTL_196` reader"]
pub type R = crate::R<DdrDenaliCtl196Spec>;
#[doc = "Register `DDR_DENALI_CTL_196` writer"]
pub type W = crate::W<DdrDenaliCtl196Spec>;
#[doc = "Field `CS_MAP` reader - Defines which chip selects are active."]
pub type CsMapR = crate::FieldReader;
#[doc = "Field `CS_MAP` writer - Defines which chip selects are active."]
pub type CsMapW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `BURST_ON_FLY_BIT` reader - Identifies the burst-on-fly bit in the memory mode registers."]
pub type BurstOnFlyBitR = crate::FieldReader;
#[doc = "Field `BURST_ON_FLY_BIT` writer - Identifies the burst-on-fly bit in the memory mode registers."]
pub type BurstOnFlyBitW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `REDUC` reader - Enable the half datapath feature of the controller. Set to 1 to enable."]
pub type ReducR = crate::BitReader;
#[doc = "Field `REDUC` writer - Enable the half datapath feature of the controller. Set to 1 to enable."]
pub type ReducW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIG_ENDIAN_EN` reader - Set byte ordering as little endian or big endian. Set to 1 for big endian."]
pub type BigEndianEnR = crate::BitReader;
#[doc = "Field `BIG_ENDIAN_EN` writer - Set byte ordering as little endian or big endian. Set to 1 for big endian."]
pub type BigEndianEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - Defines which chip selects are active."]
    #[inline(always)]
    pub fn cs_map(&self) -> CsMapR {
        CsMapR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 8:11 - Identifies the burst-on-fly bit in the memory mode registers."]
    #[inline(always)]
    pub fn burst_on_fly_bit(&self) -> BurstOnFlyBitR {
        BurstOnFlyBitR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 16 - Enable the half datapath feature of the controller. Set to 1 to enable."]
    #[inline(always)]
    pub fn reduc(&self) -> ReducR {
        ReducR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 24 - Set byte ordering as little endian or big endian. Set to 1 for big endian."]
    #[inline(always)]
    pub fn big_endian_en(&self) -> BigEndianEnR {
        BigEndianEnR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Defines which chip selects are active."]
    #[inline(always)]
    #[must_use]
    pub fn cs_map(&mut self) -> CsMapW<DdrDenaliCtl196Spec> {
        CsMapW::new(self, 0)
    }
    #[doc = "Bits 8:11 - Identifies the burst-on-fly bit in the memory mode registers."]
    #[inline(always)]
    #[must_use]
    pub fn burst_on_fly_bit(&mut self) -> BurstOnFlyBitW<DdrDenaliCtl196Spec> {
        BurstOnFlyBitW::new(self, 8)
    }
    #[doc = "Bit 16 - Enable the half datapath feature of the controller. Set to 1 to enable."]
    #[inline(always)]
    #[must_use]
    pub fn reduc(&mut self) -> ReducW<DdrDenaliCtl196Spec> {
        ReducW::new(self, 16)
    }
    #[doc = "Bit 24 - Set byte ordering as little endian or big endian. Set to 1 for big endian."]
    #[inline(always)]
    #[must_use]
    pub fn big_endian_en(&mut self) -> BigEndianEnW<DdrDenaliCtl196Spec> {
        BigEndianEnW::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_ctl_196::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_denali_ctl_196::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliCtl196Spec;
impl crate::RegisterSpec for DdrDenaliCtl196Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_ctl_196::R`](R) reader structure"]
impl crate::Readable for DdrDenaliCtl196Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_denali_ctl_196::W`](W) writer structure"]
impl crate::Writable for DdrDenaliCtl196Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_DENALI_CTL_196 to value 0"]
impl crate::Resettable for DdrDenaliCtl196Spec {
    const RESET_VALUE: u32 = 0;
}
