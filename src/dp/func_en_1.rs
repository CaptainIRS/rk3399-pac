#[doc = "Register `FUNC_EN_1` reader"]
pub type R = crate::R<FuncEn1Spec>;
#[doc = "Register `FUNC_EN_1` writer"]
pub type W = crate::W<FuncEn1Spec>;
#[doc = "Software defined function enable.\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SwFuncEnN {
    #[doc = "0: Normal operation,"]
    B0 = 0,
    #[doc = "1: Disable All the function modules. The bit has the highest priority, if the bit is 1, other function enable bits does not work."]
    B1 = 1,
}
impl From<SwFuncEnN> for bool {
    #[inline(always)]
    fn from(variant: SwFuncEnN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SW_FUNC_EN_N` reader - Software defined function enable."]
pub type SwFuncEnNR = crate::BitReader<SwFuncEnN>;
impl SwFuncEnNR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SwFuncEnN {
        match self.bits {
            false => SwFuncEnN::B0,
            true => SwFuncEnN::B1,
        }
    }
    #[doc = "Normal operation,"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == SwFuncEnN::B0
    }
    #[doc = "Disable All the function modules. The bit has the highest priority, if the bit is 1, other function enable bits does not work."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == SwFuncEnN::B1
    }
}
#[doc = "Field `SW_FUNC_EN_N` writer - Software defined function enable."]
pub type SwFuncEnNW<'a, REG> = crate::BitWriter1C<'a, REG, SwFuncEnN>;
impl<'a, REG> SwFuncEnNW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal operation,"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(SwFuncEnN::B0)
    }
    #[doc = "Disable All the function modules. The bit has the highest priority, if the bit is 1, other function enable bits does not work."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(SwFuncEnN::B1)
    }
}
#[doc = "Video FIFO functions enable.\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VidFifoFuncEnN {
    #[doc = "0: Normal operation,"]
    B0 = 0,
    #[doc = "1: Disable video FIFO."]
    B1 = 1,
}
impl From<VidFifoFuncEnN> for bool {
    #[inline(always)]
    fn from(variant: VidFifoFuncEnN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VID_FIFO_FUNC_EN_N` reader - Video FIFO functions enable."]
pub type VidFifoFuncEnNR = crate::BitReader<VidFifoFuncEnN>;
impl VidFifoFuncEnNR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> VidFifoFuncEnN {
        match self.bits {
            false => VidFifoFuncEnN::B0,
            true => VidFifoFuncEnN::B1,
        }
    }
    #[doc = "Normal operation,"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == VidFifoFuncEnN::B0
    }
    #[doc = "Disable video FIFO."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == VidFifoFuncEnN::B1
    }
}
#[doc = "Field `VID_FIFO_FUNC_EN_N` writer - Video FIFO functions enable."]
pub type VidFifoFuncEnNW<'a, REG> = crate::BitWriter1C<'a, REG, VidFifoFuncEnN>;
impl<'a, REG> VidFifoFuncEnNW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal operation,"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(VidFifoFuncEnN::B0)
    }
    #[doc = "Disable video FIFO."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(VidFifoFuncEnN::B1)
    }
}
#[doc = "Video capture functions enable.\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VidCapFuncEnN {
    #[doc = "0: Normal operation,"]
    B0 = 0,
    #[doc = "1: Disable video capture."]
    B1 = 1,
}
impl From<VidCapFuncEnN> for bool {
    #[inline(always)]
    fn from(variant: VidCapFuncEnN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VID_CAP_FUNC_EN_N` reader - Video capture functions enable."]
pub type VidCapFuncEnNR = crate::BitReader<VidCapFuncEnN>;
impl VidCapFuncEnNR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> VidCapFuncEnN {
        match self.bits {
            false => VidCapFuncEnN::B0,
            true => VidCapFuncEnN::B1,
        }
    }
    #[doc = "Normal operation,"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == VidCapFuncEnN::B0
    }
    #[doc = "Disable video capture."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == VidCapFuncEnN::B1
    }
}
#[doc = "Field `VID_CAP_FUNC_EN_N` writer - Video capture functions enable."]
pub type VidCapFuncEnNW<'a, REG> = crate::BitWriter1C<'a, REG, VidCapFuncEnN>;
impl<'a, REG> VidCapFuncEnNW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal operation,"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(VidCapFuncEnN::B0)
    }
    #[doc = "Disable video capture."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(VidCapFuncEnN::B1)
    }
}
impl R {
    #[doc = "Bit 0 - Software defined function enable."]
    #[inline(always)]
    pub fn sw_func_en_n(&self) -> SwFuncEnNR {
        SwFuncEnNR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 5 - Video FIFO functions enable."]
    #[inline(always)]
    pub fn vid_fifo_func_en_n(&self) -> VidFifoFuncEnNR {
        VidFifoFuncEnNR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Video capture functions enable."]
    #[inline(always)]
    pub fn vid_cap_func_en_n(&self) -> VidCapFuncEnNR {
        VidCapFuncEnNR::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Software defined function enable."]
    #[inline(always)]
    #[must_use]
    pub fn sw_func_en_n(&mut self) -> SwFuncEnNW<FuncEn1Spec> {
        SwFuncEnNW::new(self, 0)
    }
    #[doc = "Bit 5 - Video FIFO functions enable."]
    #[inline(always)]
    #[must_use]
    pub fn vid_fifo_func_en_n(&mut self) -> VidFifoFuncEnNW<FuncEn1Spec> {
        VidFifoFuncEnNW::new(self, 5)
    }
    #[doc = "Bit 6 - Video capture functions enable."]
    #[inline(always)]
    #[must_use]
    pub fn vid_cap_func_en_n(&mut self) -> VidCapFuncEnNW<FuncEn1Spec> {
        VidCapFuncEnNW::new(self, 6)
    }
}
#[doc = "Function Enable Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func_en_1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func_en_1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FuncEn1Spec;
impl crate::RegisterSpec for FuncEn1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`func_en_1::R`](R) reader structure"]
impl crate::Readable for FuncEn1Spec {}
#[doc = "`write(|w| ..)` method takes [`func_en_1::W`](W) writer structure"]
impl crate::Writable for FuncEn1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x61;
}
#[doc = "`reset()` method sets FUNC_EN_1 to value 0x7d"]
impl crate::Resettable for FuncEn1Spec {
    const RESET_VALUE: u32 = 0x7d;
}
