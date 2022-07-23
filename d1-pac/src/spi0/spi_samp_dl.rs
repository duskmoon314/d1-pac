#[doc = "Register `spi_samp_dl` reader"]
pub struct R(crate::R<SPI_SAMP_DL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPI_SAMP_DL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPI_SAMP_DL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPI_SAMP_DL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `spi_samp_dl` writer"]
pub struct W(crate::W<SPI_SAMP_DL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPI_SAMP_DL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<SPI_SAMP_DL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SPI_SAMP_DL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `samp_dl_cal_start` reader - Sample Delay Calibration Start"]
pub type SAMP_DL_CAL_START_R = crate::BitReader<bool>;
#[doc = "Field `samp_dl_cal_start` writer - Sample Delay Calibration Start"]
pub type SAMP_DL_CAL_START_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SPI_SAMP_DL_SPEC, bool, O>;
#[doc = "Field `samp_dl_cal_done` reader - Sample Delay Calibration Dont"]
pub type SAMP_DL_CAL_DONE_R = crate::BitReader<bool>;
#[doc = "Field `samp_dl` reader - Sample Delay"]
pub type SAMP_DL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `samp_dl_sw_en` reader - Sample Delay Software Enable"]
pub type SAMP_DL_SW_EN_R = crate::BitReader<bool>;
#[doc = "Field `samp_dl_sw_en` writer - Sample Delay Software Enable"]
pub type SAMP_DL_SW_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPI_SAMP_DL_SPEC, bool, O>;
#[doc = "Field `samp_dl_sw` reader - Sample Delay Software"]
pub type SAMP_DL_SW_R = crate::FieldReader<u8, u8>;
#[doc = "Field `samp_dl_sw` writer - Sample Delay Software"]
pub type SAMP_DL_SW_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SPI_SAMP_DL_SPEC, u8, u8, 6, O>;
impl R {
    #[doc = "Bit 15 - Sample Delay Calibration Start"]
    #[inline(always)]
    pub fn samp_dl_cal_start(&self) -> SAMP_DL_CAL_START_R {
        SAMP_DL_CAL_START_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 14 - Sample Delay Calibration Dont"]
    #[inline(always)]
    pub fn samp_dl_cal_done(&self) -> SAMP_DL_CAL_DONE_R {
        SAMP_DL_CAL_DONE_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 8:13 - Sample Delay"]
    #[inline(always)]
    pub fn samp_dl(&self) -> SAMP_DL_R {
        SAMP_DL_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bit 7 - Sample Delay Software Enable"]
    #[inline(always)]
    pub fn samp_dl_sw_en(&self) -> SAMP_DL_SW_EN_R {
        SAMP_DL_SW_EN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 0:5 - Sample Delay Software"]
    #[inline(always)]
    pub fn samp_dl_sw(&self) -> SAMP_DL_SW_R {
        SAMP_DL_SW_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bit 15 - Sample Delay Calibration Start"]
    #[inline(always)]
    pub fn samp_dl_cal_start(&mut self) -> SAMP_DL_CAL_START_W<15> {
        SAMP_DL_CAL_START_W::new(self)
    }
    #[doc = "Bit 7 - Sample Delay Software Enable"]
    #[inline(always)]
    pub fn samp_dl_sw_en(&mut self) -> SAMP_DL_SW_EN_W<7> {
        SAMP_DL_SW_EN_W::new(self)
    }
    #[doc = "Bits 0:5 - Sample Delay Software"]
    #[inline(always)]
    pub fn samp_dl_sw(&mut self) -> SAMP_DL_SW_W<0> {
        SAMP_DL_SW_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI Sample Delay Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_samp_dl](index.html) module"]
pub struct SPI_SAMP_DL_SPEC;
impl crate::RegisterSpec for SPI_SAMP_DL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [spi_samp_dl::R](R) reader structure"]
impl crate::Readable for SPI_SAMP_DL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [spi_samp_dl::W](W) writer structure"]
impl crate::Writable for SPI_SAMP_DL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets spi_samp_dl to value 0"]
impl crate::Resettable for SPI_SAMP_DL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
