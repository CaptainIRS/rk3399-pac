#[doc = "Register `SWREG0` reader"]
pub type R = crate::R<Swreg0Spec>;
#[doc = "Register `SWREG0` writer"]
pub type W = crate::W<Swreg0Spec>;
#[doc = "Field `SW_MAX_BURST_LEN` reader - the max burst length can be used by axi bus\n\nrange : 1-16"]
pub type SwMaxBurstLenR = crate::FieldReader;
#[doc = "Field `SW_MAX_BURST_LEN` writer - the max burst length can be used by axi bus\n\nrange : 1-16"]
pub type SwMaxBurstLenW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "on-off for AXI Single Command Multiple Data\n\non-off for AXI Single Command Multiple Data\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SwScmdOff {
    #[doc = "0: on"]
    B0 = 0,
    #[doc = "1: off"]
    B1 = 1,
}
impl From<SwScmdOff> for bool {
    #[inline(always)]
    fn from(variant: SwScmdOff) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SW_SCMD_OFF` reader - on-off for AXI Single Command Multiple Data\n\non-off for AXI Single Command Multiple Data"]
pub type SwScmdOffR = crate::BitReader<SwScmdOff>;
impl SwScmdOffR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SwScmdOff {
        match self.bits {
            false => SwScmdOff::B0,
            true => SwScmdOff::B1,
        }
    }
    #[doc = "on"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == SwScmdOff::B0
    }
    #[doc = "off"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == SwScmdOff::B1
    }
}
#[doc = "Field `SW_SCMD_OFF` writer - on-off for AXI Single Command Multiple Data\n\non-off for AXI Single Command Multiple Data"]
pub type SwScmdOffW<'a, REG> = crate::BitWriter<'a, REG, SwScmdOff>;
impl<'a, REG> SwScmdOffW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "on"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(SwScmdOff::B0)
    }
    #[doc = "off"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(SwScmdOff::B1)
    }
}
#[doc = "Field `SW_AXI_ID_RD` reader - AXI Read ID\n\nif you config 0,will modify as 1 by hw"]
pub type SwAxiIdRdR = crate::FieldReader;
#[doc = "Field `SW_AXI_ID_RD` writer - AXI Read ID\n\nif you config 0,will modify as 1 by hw"]
pub type SwAxiIdRdW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SW_AXI_ID_WR` reader - AXI Write ID\n\nif you config 0,will modify as 1 by hw"]
pub type SwAxiIdWrR = crate::FieldReader;
#[doc = "Field `SW_AXI_ID_WR` writer - AXI Write ID\n\nif you config 0,will modify as 1 by hw"]
pub type SwAxiIdWrW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:4 - the max burst length can be used by axi bus\n\nrange : 1-16"]
    #[inline(always)]
    pub fn sw_max_burst_len(&self) -> SwMaxBurstLenR {
        SwMaxBurstLenR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 5 - on-off for AXI Single Command Multiple Data\n\non-off for AXI Single Command Multiple Data"]
    #[inline(always)]
    pub fn sw_scmd_off(&self) -> SwScmdOffR {
        SwScmdOffR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 8:15 - AXI Read ID\n\nif you config 0,will modify as 1 by hw"]
    #[inline(always)]
    pub fn sw_axi_id_rd(&self) -> SwAxiIdRdR {
        SwAxiIdRdR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - AXI Write ID\n\nif you config 0,will modify as 1 by hw"]
    #[inline(always)]
    pub fn sw_axi_id_wr(&self) -> SwAxiIdWrR {
        SwAxiIdWrR::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - the max burst length can be used by axi bus\n\nrange : 1-16"]
    #[inline(always)]
    #[must_use]
    pub fn sw_max_burst_len(&mut self) -> SwMaxBurstLenW<Swreg0Spec> {
        SwMaxBurstLenW::new(self, 0)
    }
    #[doc = "Bit 5 - on-off for AXI Single Command Multiple Data\n\non-off for AXI Single Command Multiple Data"]
    #[inline(always)]
    #[must_use]
    pub fn sw_scmd_off(&mut self) -> SwScmdOffW<Swreg0Spec> {
        SwScmdOffW::new(self, 5)
    }
    #[doc = "Bits 8:15 - AXI Read ID\n\nif you config 0,will modify as 1 by hw"]
    #[inline(always)]
    #[must_use]
    pub fn sw_axi_id_rd(&mut self) -> SwAxiIdRdW<Swreg0Spec> {
        SwAxiIdRdW::new(self, 8)
    }
    #[doc = "Bits 16:23 - AXI Write ID\n\nif you config 0,will modify as 1 by hw"]
    #[inline(always)]
    #[must_use]
    pub fn sw_axi_id_wr(&mut self) -> SwAxiIdWrW<Swreg0Spec> {
        SwAxiIdWrW::new(self, 16)
    }
}
#[doc = "axi control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg0Spec;
impl crate::RegisterSpec for Swreg0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg0::R`](R) reader structure"]
impl crate::Readable for Swreg0Spec {}
#[doc = "`write(|w| ..)` method takes [`swreg0::W`](W) writer structure"]
impl crate::Writable for Swreg0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG0 to value 0x0001_0100"]
impl crate::Resettable for Swreg0Spec {
    const RESET_VALUE: u32 = 0x0001_0100;
}
