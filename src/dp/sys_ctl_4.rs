#[doc = "Register `SYS_CTL_4` reader"]
pub type R = crate::R<SysCtl4Spec>;
#[doc = "Register `SYS_CTL_4` writer"]
pub type W = crate::W<SysCtl4Spec>;
#[doc = "Control M_VID update frequency\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MVidUpdateCtrl {
    #[doc = "3: 1/8 X update rate,"]
    B11 = 3,
    #[doc = "2: 1/4 X update rate,"]
    B10 = 2,
    #[doc = "1: 1/2 X update rate,"]
    B01 = 1,
    #[doc = "0: Normal rate."]
    B00 = 0,
}
impl From<MVidUpdateCtrl> for u8 {
    #[inline(always)]
    fn from(variant: MVidUpdateCtrl) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MVidUpdateCtrl {
    type Ux = u8;
}
#[doc = "Field `M_VID_UPDATE_CTRL` reader - Control M_VID update frequency"]
pub type MVidUpdateCtrlR = crate::FieldReader<MVidUpdateCtrl>;
impl MVidUpdateCtrlR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MVidUpdateCtrl {
        match self.bits {
            3 => MVidUpdateCtrl::B11,
            2 => MVidUpdateCtrl::B10,
            1 => MVidUpdateCtrl::B01,
            0 => MVidUpdateCtrl::B00,
            _ => unreachable!(),
        }
    }
    #[doc = "1/8 X update rate,"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == MVidUpdateCtrl::B11
    }
    #[doc = "1/4 X update rate,"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == MVidUpdateCtrl::B10
    }
    #[doc = "1/2 X update rate,"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == MVidUpdateCtrl::B01
    }
    #[doc = "Normal rate."]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == MVidUpdateCtrl::B00
    }
}
#[doc = "Field `M_VID_UPDATE_CTRL` writer - Control M_VID update frequency"]
pub type MVidUpdateCtrlW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, MVidUpdateCtrl>;
impl<'a, REG> MVidUpdateCtrlW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "1/8 X update rate,"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(MVidUpdateCtrl::B11)
    }
    #[doc = "1/4 X update rate,"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(MVidUpdateCtrl::B10)
    }
    #[doc = "1/2 X update rate,"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(MVidUpdateCtrl::B01)
    }
    #[doc = "Normal rate."]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(MVidUpdateCtrl::B00)
    }
}
#[doc = "Fix M_VID value\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FixMVid {
    #[doc = "1: Use register M_VID value to be sent out,"]
    B1 = 1,
    #[doc = "0: Use calculates M_VID value to be sent out."]
    B0 = 0,
}
impl From<FixMVid> for bool {
    #[inline(always)]
    fn from(variant: FixMVid) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FIX_M_VID` reader - Fix M_VID value"]
pub type FixMVidR = crate::BitReader<FixMVid>;
impl FixMVidR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FixMVid {
        match self.bits {
            true => FixMVid::B1,
            false => FixMVid::B0,
        }
    }
    #[doc = "Use register M_VID value to be sent out,"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == FixMVid::B1
    }
    #[doc = "Use calculates M_VID value to be sent out."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == FixMVid::B0
    }
}
#[doc = "Field `FIX_M_VID` writer - Fix M_VID value"]
pub type FixMVidW<'a, REG> = crate::BitWriter<'a, REG, FixMVid>;
impl<'a, REG> FixMVidW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Use register M_VID value to be sent out,"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(FixMVid::B1)
    }
    #[doc = "Use calculates M_VID value to be sent out."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(FixMVid::B0)
    }
}
#[doc = "DisplayPort Enhanced mode enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enhanced {
    #[doc = "1: Enhanced mode,"]
    B1 = 1,
    #[doc = "0: Normal mode."]
    B0 = 0,
}
impl From<Enhanced> for bool {
    #[inline(always)]
    fn from(variant: Enhanced) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENHANCED` reader - DisplayPort Enhanced mode enable"]
pub type EnhancedR = crate::BitReader<Enhanced>;
impl EnhancedR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Enhanced {
        match self.bits {
            true => Enhanced::B1,
            false => Enhanced::B0,
        }
    }
    #[doc = "Enhanced mode,"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Enhanced::B1
    }
    #[doc = "Normal mode."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Enhanced::B0
    }
}
#[doc = "Field `ENHANCED` writer - DisplayPort Enhanced mode enable"]
pub type EnhancedW<'a, REG> = crate::BitWriter1C<'a, REG, Enhanced>;
impl<'a, REG> EnhancedW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enhanced mode,"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Enhanced::B1)
    }
    #[doc = "Normal mode."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Enhanced::B0)
    }
}
impl R {
    #[doc = "Bits 0:1 - Control M_VID update frequency"]
    #[inline(always)]
    pub fn m_vid_update_ctrl(&self) -> MVidUpdateCtrlR {
        MVidUpdateCtrlR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - Fix M_VID value"]
    #[inline(always)]
    pub fn fix_m_vid(&self) -> FixMVidR {
        FixMVidR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DisplayPort Enhanced mode enable"]
    #[inline(always)]
    pub fn enhanced(&self) -> EnhancedR {
        EnhancedR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Control M_VID update frequency"]
    #[inline(always)]
    #[must_use]
    pub fn m_vid_update_ctrl(&mut self) -> MVidUpdateCtrlW<SysCtl4Spec> {
        MVidUpdateCtrlW::new(self, 0)
    }
    #[doc = "Bit 2 - Fix M_VID value"]
    #[inline(always)]
    #[must_use]
    pub fn fix_m_vid(&mut self) -> FixMVidW<SysCtl4Spec> {
        FixMVidW::new(self, 2)
    }
    #[doc = "Bit 3 - DisplayPort Enhanced mode enable"]
    #[inline(always)]
    #[must_use]
    pub fn enhanced(&mut self) -> EnhancedW<SysCtl4Spec> {
        EnhancedW::new(self, 3)
    }
}
#[doc = "System Control Register #4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sys_ctl_4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sys_ctl_4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SysCtl4Spec;
impl crate::RegisterSpec for SysCtl4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sys_ctl_4::R`](R) reader structure"]
impl crate::Readable for SysCtl4Spec {}
#[doc = "`write(|w| ..)` method takes [`sys_ctl_4::W`](W) writer structure"]
impl crate::Writable for SysCtl4Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x0b;
}
#[doc = "`reset()` method sets SYS_CTL_4 to value 0"]
impl crate::Resettable for SysCtl4Spec {
    const RESET_VALUE: u32 = 0;
}
