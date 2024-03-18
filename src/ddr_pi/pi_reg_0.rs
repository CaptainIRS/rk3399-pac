#[doc = "Register `PI_REG_0` reader"]
pub type R = crate::R<PiReg0Spec>;
#[doc = "Register `PI_REG_0` writer"]
pub type W = crate::W<PiReg0Spec>;
#[doc = "Field `PI_START` reader - Initiates command processing in the PI. Set to 1 to initiate."]
pub type PiStartR = crate::BitReader;
#[doc = "Field `PI_START` writer - Initiates command processing in the PI. Set to 1 to initiate."]
pub type PiStartW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Defines the mode of operation of the PI.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PiDramClass {
    #[doc = "6: LPDDR4 other value: reserved"]
    B0110 = 6,
    #[doc = "7: LPDDR4 other value: reserved"]
    B0111 = 7,
    #[doc = "11: LPDDR4 other value: reserved"]
    B1011 = 11,
}
impl From<PiDramClass> for u8 {
    #[inline(always)]
    fn from(variant: PiDramClass) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PiDramClass {
    type Ux = u8;
}
#[doc = "Field `PI_DRAM_CLASS` reader - Defines the mode of operation of the PI."]
pub type PiDramClassR = crate::FieldReader<PiDramClass>;
impl PiDramClassR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<PiDramClass> {
        match self.bits {
            6 => Some(PiDramClass::B0110),
            7 => Some(PiDramClass::B0111),
            11 => Some(PiDramClass::B1011),
            _ => None,
        }
    }
    #[doc = "LPDDR4 other value: reserved"]
    #[inline(always)]
    pub fn is_b0110(&self) -> bool {
        *self == PiDramClass::B0110
    }
    #[doc = "LPDDR4 other value: reserved"]
    #[inline(always)]
    pub fn is_b0111(&self) -> bool {
        *self == PiDramClass::B0111
    }
    #[doc = "LPDDR4 other value: reserved"]
    #[inline(always)]
    pub fn is_b1011(&self) -> bool {
        *self == PiDramClass::B1011
    }
}
#[doc = "Field `PI_DRAM_CLASS` writer - Defines the mode of operation of the PI."]
pub type PiDramClassW<'a, REG> = crate::FieldWriter<'a, REG, 4, PiDramClass>;
impl<'a, REG> PiDramClassW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "LPDDR4 other value: reserved"]
    #[inline(always)]
    pub fn b0110(self) -> &'a mut crate::W<REG> {
        self.variant(PiDramClass::B0110)
    }
    #[doc = "LPDDR4 other value: reserved"]
    #[inline(always)]
    pub fn b0111(self) -> &'a mut crate::W<REG> {
        self.variant(PiDramClass::B0111)
    }
    #[doc = "LPDDR4 other value: reserved"]
    #[inline(always)]
    pub fn b1011(self) -> &'a mut crate::W<REG> {
        self.variant(PiDramClass::B1011)
    }
}
#[doc = "Field `PI_VERSION` reader - Holds the PI version number."]
pub type PiVersionR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bit 0 - Initiates command processing in the PI. Set to 1 to initiate."]
    #[inline(always)]
    pub fn pi_start(&self) -> PiStartR {
        PiStartR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 8:11 - Defines the mode of operation of the PI."]
    #[inline(always)]
    pub fn pi_dram_class(&self) -> PiDramClassR {
        PiDramClassR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:31 - Holds the PI version number."]
    #[inline(always)]
    pub fn pi_version(&self) -> PiVersionR {
        PiVersionR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - Initiates command processing in the PI. Set to 1 to initiate."]
    #[inline(always)]
    #[must_use]
    pub fn pi_start(&mut self) -> PiStartW<PiReg0Spec> {
        PiStartW::new(self, 0)
    }
    #[doc = "Bits 8:11 - Defines the mode of operation of the PI."]
    #[inline(always)]
    #[must_use]
    pub fn pi_dram_class(&mut self) -> PiDramClassW<PiReg0Spec> {
        PiDramClassW::new(self, 8)
    }
}
#[doc = "DDR PHY Independent Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pi_reg_0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pi_reg_0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PiReg0Spec;
impl crate::RegisterSpec for PiReg0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pi_reg_0::R`](R) reader structure"]
impl crate::Readable for PiReg0Spec {}
#[doc = "`write(|w| ..)` method takes [`pi_reg_0::W`](W) writer structure"]
impl crate::Writable for PiReg0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PI_REG_0 to value 0"]
impl crate::Resettable for PiReg0Spec {
    const RESET_VALUE: u32 = 0;
}
