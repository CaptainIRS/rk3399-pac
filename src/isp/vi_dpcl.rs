#[doc = "Register `VI_DPCL` reader"]
pub type R = crate::R<ViDpclSpec>;
#[doc = "Register `VI_DPCL` writer"]
pub type W = crate::W<ViDpclSpec>;
#[doc = "data path selector for main path\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ViMpMux {
    #[doc = "0: data from DMA read port to JPEG encoder"]
    B00 = 0,
    #[doc = "1: data from main resize to MI, uncompressed"]
    B01 = 1,
    #[doc = "2: data from main resize to JPEG encoder"]
    B10 = 2,
}
impl From<ViMpMux> for u8 {
    #[inline(always)]
    fn from(variant: ViMpMux) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ViMpMux {
    type Ux = u8;
}
#[doc = "Field `vi_mp_mux` reader - data path selector for main path"]
pub type ViMpMuxR = crate::FieldReader<ViMpMux>;
impl ViMpMuxR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<ViMpMux> {
        match self.bits {
            0 => Some(ViMpMux::B00),
            1 => Some(ViMpMux::B01),
            2 => Some(ViMpMux::B10),
            _ => None,
        }
    }
    #[doc = "data from DMA read port to JPEG encoder"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == ViMpMux::B00
    }
    #[doc = "data from main resize to MI, uncompressed"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == ViMpMux::B01
    }
    #[doc = "data from main resize to JPEG encoder"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == ViMpMux::B10
    }
}
#[doc = "Field `vi_mp_mux` writer - data path selector for main path"]
pub type ViMpMuxW<'a, REG> = crate::FieldWriter<'a, REG, 2, ViMpMux>;
impl<'a, REG> ViMpMuxW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "data from DMA read port to JPEG encoder"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(ViMpMux::B00)
    }
    #[doc = "data from main resize to MI, uncompressed"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(ViMpMux::B01)
    }
    #[doc = "data from main resize to JPEG encoder"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(ViMpMux::B10)
    }
}
#[doc = "selects input interface\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum IfSelect {
    #[doc = "0: parallel interface"]
    D0 = 0,
    #[doc = "1: SMIA-interface"]
    D1 = 1,
    #[doc = "2: MIPI1-interface"]
    D2 = 2,
}
impl From<IfSelect> for u8 {
    #[inline(always)]
    fn from(variant: IfSelect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for IfSelect {
    type Ux = u8;
}
#[doc = "Field `if_select` reader - selects input interface"]
pub type IfSelectR = crate::FieldReader<IfSelect>;
impl IfSelectR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<IfSelect> {
        match self.bits {
            0 => Some(IfSelect::D0),
            1 => Some(IfSelect::D1),
            2 => Some(IfSelect::D2),
            _ => None,
        }
    }
    #[doc = "parallel interface"]
    #[inline(always)]
    pub fn is_d0(&self) -> bool {
        *self == IfSelect::D0
    }
    #[doc = "SMIA-interface"]
    #[inline(always)]
    pub fn is_d1(&self) -> bool {
        *self == IfSelect::D1
    }
    #[doc = "MIPI1-interface"]
    #[inline(always)]
    pub fn is_d2(&self) -> bool {
        *self == IfSelect::D2
    }
}
#[doc = "Field `if_select` writer - selects input interface"]
pub type IfSelectW<'a, REG> = crate::FieldWriter<'a, REG, 2, IfSelect>;
impl<'a, REG> IfSelectW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "parallel interface"]
    #[inline(always)]
    pub fn d0(self) -> &'a mut crate::W<REG> {
        self.variant(IfSelect::D0)
    }
    #[doc = "SMIA-interface"]
    #[inline(always)]
    pub fn d1(self) -> &'a mut crate::W<REG> {
        self.variant(IfSelect::D1)
    }
    #[doc = "MIPI1-interface"]
    #[inline(always)]
    pub fn d2(self) -> &'a mut crate::W<REG> {
        self.variant(IfSelect::D2)
    }
}
impl R {
    #[doc = "Bits 0:1 - data path selector for main path"]
    #[inline(always)]
    pub fn vi_mp_mux(&self) -> ViMpMuxR {
        ViMpMuxR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 8:9 - selects input interface"]
    #[inline(always)]
    pub fn if_select(&self) -> IfSelectR {
        IfSelectR::new(((self.bits >> 8) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - data path selector for main path"]
    #[inline(always)]
    #[must_use]
    pub fn vi_mp_mux(&mut self) -> ViMpMuxW<ViDpclSpec> {
        ViMpMuxW::new(self, 0)
    }
    #[doc = "Bits 8:9 - selects input interface"]
    #[inline(always)]
    #[must_use]
    pub fn if_select(&mut self) -> IfSelectW<ViDpclSpec> {
        IfSelectW::new(self, 8)
    }
}
#[doc = "Data path control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vi_dpcl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vi_dpcl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ViDpclSpec;
impl crate::RegisterSpec for ViDpclSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vi_dpcl::R`](R) reader structure"]
impl crate::Readable for ViDpclSpec {}
#[doc = "`write(|w| ..)` method takes [`vi_dpcl::W`](W) writer structure"]
impl crate::Writable for ViDpclSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets VI_DPCL to value 0"]
impl crate::Resettable for ViDpclSpec {
    const RESET_VALUE: u32 = 0;
}
