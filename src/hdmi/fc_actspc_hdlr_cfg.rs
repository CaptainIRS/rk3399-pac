#[doc = "Register `FC_ACTSPC_HDLR_CFG` reader"]
pub type R = crate::R<FcActspcHdlrCfgSpec>;
#[doc = "Register `FC_ACTSPC_HDLR_CFG` writer"]
pub type W = crate::W<FcActspcHdlrCfgSpec>;
#[doc = "Active Space Handler Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ActspcHdlrEn {
    #[doc = "1: Fixed active space value mode disabled"]
    B1 = 1,
    #[doc = "0: Fixed active space value mode disabled"]
    B0 = 0,
}
impl From<ActspcHdlrEn> for bool {
    #[inline(always)]
    fn from(variant: ActspcHdlrEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ACTSPC_HDLR_EN` reader - Active Space Handler Control"]
pub type ActspcHdlrEnR = crate::BitReader<ActspcHdlrEn>;
impl ActspcHdlrEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ActspcHdlrEn {
        match self.bits {
            true => ActspcHdlrEn::B1,
            false => ActspcHdlrEn::B0,
        }
    }
    #[doc = "Fixed active space value mode disabled"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == ActspcHdlrEn::B1
    }
    #[doc = "Fixed active space value mode disabled"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == ActspcHdlrEn::B0
    }
}
#[doc = "Field `ACTSPC_HDLR_EN` writer - Active Space Handler Control"]
pub type ActspcHdlrEnW<'a, REG> = crate::BitWriter<'a, REG, ActspcHdlrEn>;
impl<'a, REG> ActspcHdlrEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Fixed active space value mode disabled"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(ActspcHdlrEn::B1)
    }
    #[doc = "Fixed active space value mode disabled"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(ActspcHdlrEn::B0)
    }
}
#[doc = "Active Space handler control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ActspcHdlrTgl {
    #[doc = "1: Active space not oscillating"]
    B1 = 1,
    #[doc = "0: Active space not oscillating"]
    B0 = 0,
}
impl From<ActspcHdlrTgl> for bool {
    #[inline(always)]
    fn from(variant: ActspcHdlrTgl) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ACTSPC_HDLR_TGL` reader - Active Space handler control"]
pub type ActspcHdlrTglR = crate::BitReader<ActspcHdlrTgl>;
impl ActspcHdlrTglR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ActspcHdlrTgl {
        match self.bits {
            true => ActspcHdlrTgl::B1,
            false => ActspcHdlrTgl::B0,
        }
    }
    #[doc = "Active space not oscillating"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == ActspcHdlrTgl::B1
    }
    #[doc = "Active space not oscillating"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == ActspcHdlrTgl::B0
    }
}
#[doc = "Field `ACTSPC_HDLR_TGL` writer - Active Space handler control"]
pub type ActspcHdlrTglW<'a, REG> = crate::BitWriter<'a, REG, ActspcHdlrTgl>;
impl<'a, REG> ActspcHdlrTglW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Active space not oscillating"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(ActspcHdlrTgl::B1)
    }
    #[doc = "Active space not oscillating"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(ActspcHdlrTgl::B0)
    }
}
impl R {
    #[doc = "Bit 0 - Active Space Handler Control"]
    #[inline(always)]
    pub fn actspc_hdlr_en(&self) -> ActspcHdlrEnR {
        ActspcHdlrEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Active Space handler control"]
    #[inline(always)]
    pub fn actspc_hdlr_tgl(&self) -> ActspcHdlrTglR {
        ActspcHdlrTglR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Active Space Handler Control"]
    #[inline(always)]
    #[must_use]
    pub fn actspc_hdlr_en(&mut self) -> ActspcHdlrEnW<FcActspcHdlrCfgSpec> {
        ActspcHdlrEnW::new(self, 0)
    }
    #[doc = "Bit 1 - Active Space handler control"]
    #[inline(always)]
    #[must_use]
    pub fn actspc_hdlr_tgl(&mut self) -> ActspcHdlrTglW<FcActspcHdlrCfgSpec> {
        ActspcHdlrTglW::new(self, 1)
    }
}
#[doc = "Active Space Handler Control 1b: Fixed active space value mode enabled. During active space, a fixed value of 0xAA is applied to all TMDS channels. 0b: Fixed active space value mode disabled\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_actspc_hdlr_cfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_actspc_hdlr_cfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FcActspcHdlrCfgSpec;
impl crate::RegisterSpec for FcActspcHdlrCfgSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`fc_actspc_hdlr_cfg::R`](R) reader structure"]
impl crate::Readable for FcActspcHdlrCfgSpec {}
#[doc = "`write(|w| ..)` method takes [`fc_actspc_hdlr_cfg::W`](W) writer structure"]
impl crate::Writable for FcActspcHdlrCfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets FC_ACTSPC_HDLR_CFG to value 0"]
impl crate::Resettable for FcActspcHdlrCfgSpec {
    const RESET_VALUE: u8 = 0;
}
